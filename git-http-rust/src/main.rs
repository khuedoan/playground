use std::{net::SocketAddr, path::PathBuf, process::Stdio};

use axum::{
    body::Bytes,
    extract::{OriginalUri, Path, State},
    http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode},
    response::{IntoResponse, Response},
    routing::{any, get},
    Router,
};
use clap::Parser;
use tokio::{io::AsyncWriteExt, process::Command};
use tracing::{error, info};

#[derive(Debug, Clone)]
struct AppState {
    repos_root: PathBuf,
}

#[derive(Debug, Parser)]
#[command(
    name = "git-http-rust",
    about = "Git Smart HTTP server via git-http-backend (no auth)"
)]
struct Args {
    /// Address to bind, for example 127.0.0.1:8080
    #[arg(long, default_value = "127.0.0.1:8080")]
    listen: SocketAddr,

    /// Root directory containing bare repositories
    #[arg(long, default_value = "./repos")]
    repos_root: PathBuf,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "git_http_rust=info,axum=info".into()),
        )
        .init();

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/*path", any(git_http_backend))
        .with_state(AppState {
            repos_root: args.repos_root,
        });

    let listener = tokio::net::TcpListener::bind(args.listen)
        .await
        .expect("bind listener");

    info!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .expect("server error");
}

async fn shutdown_signal() {
    let _ = tokio::signal::ctrl_c().await;
    info!("shutdown signal received");
}

async fn healthz() -> &'static str {
    "ok"
}

async fn git_http_backend(
    State(state): State<AppState>,
    Path(path): Path<String>,
    method: Method,
    headers: HeaderMap,
    OriginalUri(uri): OriginalUri,
    body: Bytes,
) -> Response {
    let path_info = format!("/{}", path.trim_start_matches('/'));

    // git-http-backend speaks CGI over stdin/stdout.
    let mut cmd = Command::new("git");
    cmd.arg("http-backend")
        .env("GIT_PROJECT_ROOT", &state.repos_root)
        .env("GIT_HTTP_EXPORT_ALL", "")
        .env("REQUEST_METHOD", method.as_str())
        .env("PATH_INFO", &path_info)
        .env("QUERY_STRING", uri.query().unwrap_or(""))
        .env("REMOTE_ADDR", "127.0.0.1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if let Some(content_type) = headers.get("content-type") {
        if let Ok(content_type) = content_type.to_str() {
            cmd.env("CONTENT_TYPE", content_type);
        }
    }

    if !body.is_empty() {
        cmd.env("CONTENT_LENGTH", body.len().to_string());
    }

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) => {
            error!(%err, "failed to spawn git http-backend");
            return status_message(StatusCode::INTERNAL_SERVER_ERROR, "failed to execute git");
        }
    };

    if !body.is_empty() {
        if let Some(mut stdin_writer) = child.stdin.take() {
            if let Err(err) = stdin_writer.write_all(&body).await {
                error!(%err, "failed to write request body to git-http-backend");
                return status_message(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to process request body",
                );
            }
        }
    }

    let output = match child.wait_with_output().await {
        Ok(output) => output,
        Err(err) => {
            error!(%err, "git-http-backend failed to finish");
            return status_message(StatusCode::INTERNAL_SERVER_ERROR, "git process failed");
        }
    };

    if !output.status.success() {
        let stderr_msg = String::from_utf8_lossy(&output.stderr);
        error!(status = %output.status, stderr = %stderr_msg, "git-http-backend failed");
        return status_message(StatusCode::INTERNAL_SERVER_ERROR, "git http-backend failed");
    }

    match cgi_to_http_response(&output.stdout) {
        Ok(resp) => resp,
        Err(err_msg) => {
            error!(%err_msg, "failed to parse git-http-backend CGI response");
            status_message(
                StatusCode::INTERNAL_SERVER_ERROR,
                "invalid git backend response",
            )
        }
    }
}

fn cgi_to_http_response(cgi_stdout: &[u8]) -> Result<Response, String> {
    let split = cgi_stdout
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|idx| (idx, 4))
        .or_else(|| {
            cgi_stdout
                .windows(2)
                .position(|w| w == b"\n\n")
                .map(|idx| (idx, 2))
        })
        .ok_or_else(|| "missing CGI header/body separator".to_string())?;

    let (header_len, sep_len) = split;
    let header_bytes = &cgi_stdout[..header_len];
    let body = cgi_stdout[header_len + sep_len..].to_vec();

    let header_text = String::from_utf8_lossy(header_bytes);
    let mut status = StatusCode::OK;
    let mut out_headers = HeaderMap::new();

    for raw_line in header_text.lines() {
        let line = raw_line.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }

        if let Some(rest) = line.strip_prefix("Status:") {
            let code = rest
                .trim()
                .split_whitespace()
                .next()
                .ok_or_else(|| "invalid Status header".to_string())?;
            let parsed = code
                .parse::<u16>()
                .map_err(|_| "invalid Status code".to_string())?;
            status =
                StatusCode::from_u16(parsed).map_err(|_| "unsupported Status code".to_string())?;
            continue;
        }

        let Some((name, value)) = line.split_once(':') else {
            return Err(format!("invalid header line: {line}"));
        };

        let name = HeaderName::from_bytes(name.trim().as_bytes())
            .map_err(|_| format!("invalid header name: {name}"))?;
        let value = HeaderValue::from_str(value.trim())
            .map_err(|_| format!("invalid header value for {name}"))?;
        out_headers.append(name, value);
    }

    Ok((status, out_headers, body).into_response())
}

fn status_message(status: StatusCode, msg: &'static str) -> Response {
    (status, msg).into_response()
}

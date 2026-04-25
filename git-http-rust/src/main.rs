use std::{collections::HashMap, net::SocketAddr, path::PathBuf, process::Stdio};

use axum::{
    body::Bytes,
    extract::{Path, Query, State},
    http::{header, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use clap::Parser;
use tokio::{io::AsyncWriteExt, process::Command};
use tracing::{error, info, warn};

#[derive(Debug, Clone)]
struct AppState {
    repos_root: PathBuf,
}

#[derive(Debug, Parser)]
#[command(
    name = "git-http-rust",
    about = "Minimal Git Smart HTTP server (upload-pack only, no auth)"
)]
struct Args {
    /// Address to bind, for example 127.0.0.1:8080
    #[arg(long, default_value = "127.0.0.1:8080")]
    listen: SocketAddr,

    /// Root directory containing bare repositories (for example ./repos)
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
        .route("/:repo/info/refs", get(info_refs))
        .route("/:repo/git-upload-pack", post(git_upload_pack))
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

async fn info_refs(
    State(state): State<AppState>,
    Path(repo): Path<String>,
    Query(query): Query<HashMap<String, String>>,
) -> Response {
    if query.get("service").map(String::as_str) != Some("git-upload-pack") {
        return status_message(
            StatusCode::BAD_REQUEST,
            "only ?service=git-upload-pack is supported",
        );
    }

    run_upload_pack(&state.repos_root, &repo, b"").await
}

async fn git_upload_pack(
    State(state): State<AppState>,
    Path(repo): Path<String>,
    body: Bytes,
) -> Response {
    run_upload_pack(&state.repos_root, &repo, &body).await
}

async fn run_upload_pack(repos_root: &PathBuf, repo: &str, stdin: &[u8]) -> Response {
    if repo.contains("..") || repo.contains('/') || repo.contains('\\') {
        return status_message(StatusCode::BAD_REQUEST, "invalid repository name");
    }

    let repo_path = repos_root.join(repo);

    if !repo_path.exists() {
        return status_message(StatusCode::NOT_FOUND, "repository not found");
    }

    let mut cmd = Command::new("git");
    cmd.args([
        "upload-pack",
        "--stateless-rpc",
        "--advertise-refs",
        repo_path.to_string_lossy().as_ref(),
    ])
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

    let advertise = stdin.is_empty();
    if !advertise {
        cmd = {
            let mut c = Command::new("git");
            c.args([
                "upload-pack",
                "--stateless-rpc",
                repo_path.to_string_lossy().as_ref(),
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
            c
        };
    }

    let mut child = match cmd.spawn() {
        Ok(child) => child,
        Err(err) => {
            error!(%err, "failed to spawn git upload-pack");
            return status_message(StatusCode::INTERNAL_SERVER_ERROR, "failed to execute git");
        }
    };

    if !stdin.is_empty() {
        if let Some(mut stdin_writer) = child.stdin.take() {
            if let Err(err) = stdin_writer.write_all(stdin).await {
                error!(%err, "failed to write request body to upload-pack stdin");
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
            error!(%err, "upload-pack failed to finish");
            return status_message(StatusCode::INTERNAL_SERVER_ERROR, "git process failed");
        }
    };

    if !output.status.success() {
        let stderr_msg = String::from_utf8_lossy(&output.stderr);
        warn!(status = %output.status, stderr = %stderr_msg, "upload-pack returned non-zero status");
        return status_message(StatusCode::INTERNAL_SERVER_ERROR, "git upload-pack failed");
    }

    let mut headers = HeaderMap::new();
    let ctype = if advertise {
        "application/x-git-upload-pack-advertisement"
    } else {
        "application/x-git-upload-pack-result"
    };
    headers.insert(header::CONTENT_TYPE, HeaderValue::from_static(ctype));
    headers.insert(header::CACHE_CONTROL, HeaderValue::from_static("no-cache"));

    (StatusCode::OK, headers, output.stdout).into_response()
}

fn status_message(status: StatusCode, msg: &'static str) -> Response {
    (status, msg).into_response()
}

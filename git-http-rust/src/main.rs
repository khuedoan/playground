use std::{
    collections::HashMap,
    net::SocketAddr,
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::{Duration, Instant},
};

use axum::{
    body::Bytes,
    extract::{OriginalUri, Path as AxumPath, Query, State},
    http::{header, HeaderMap, HeaderName, HeaderValue, Method, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{any, get, post},
    Form, Json, Router,
};
use base64::Engine;
use chrono::{Duration as ChronoDuration, Utc};
use clap::Parser;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use tokio::{io::AsyncWriteExt, process::Command, sync::RwLock};
use tracing::{error, info};

#[derive(Debug, Clone)]
struct AppState {
    repos_root: PathBuf,
    auth: AuthConfig,
    device_codes: Arc<RwLock<HashMap<String, DeviceCodeEntry>>>,
}

#[derive(Debug, Clone)]
struct AuthConfig {
    issuer: String,
    audience: String,
    jwt_secret: String,
}

#[derive(Debug, Clone)]
struct DeviceCodeEntry {
    user_code: String,
    username: Option<String>,
    scopes: Vec<String>,
    expires_at: Instant,
    approved: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Claims {
    sub: String,
    aud: String,
    iss: String,
    exp: usize,
    iat: usize,
    scope: String,
}

#[derive(Debug, Parser)]
#[command(
    name = "git-http-rust",
    about = "Git Smart HTTP server with OAuth2 Device Flow auth"
)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:8080")]
    listen: SocketAddr,

    #[arg(long, default_value = "./repos")]
    repos_root: PathBuf,

    #[arg(long, default_value = "gx-local")]
    issuer: String,

    #[arg(long, default_value = "git-http-rust")]
    audience: String,

    #[arg(long, default_value = "dev-secret-change-me")]
    jwt_secret: String,
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

    let state = AppState {
        repos_root: args.repos_root,
        auth: AuthConfig {
            issuer: args.issuer,
            audience: args.audience,
            jwt_secret: args.jwt_secret,
        },
        device_codes: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/healthz", get(healthz))
        .route("/oauth/device/code", post(oauth_device_code))
        .route("/oauth/token", post(oauth_token))
        .route(
            "/oauth/verify",
            get(oauth_verify_page).post(oauth_verify_submit),
        )
        .route("/ui/repos", get(ui_repos))
        .route("/ui/repos/:repo/blob/*path", get(ui_blob))
        .route("/ui/repos/:repo/tree/*path", get(ui_tree))
        .route("/*path", any(git_http_backend))
        .with_state(state);

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

#[derive(Debug, Deserialize)]
struct DeviceCodeReq {
    client_id: String,
    #[serde(default)]
    scope: String,
}

#[derive(Debug, Serialize)]
struct DeviceCodeResp {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: String,
    expires_in: u64,
    interval: u64,
}

async fn oauth_device_code(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<DeviceCodeReq>,
) -> Response {
    let host = host_base_url(&headers);
    let scopes = parse_scopes(&req.scope);
    let _ = req.client_id;

    let device_code = rand_string(42);
    let user_code = rand_string(8).to_uppercase();
    let expires_in = 600;

    {
        let mut codes = state.device_codes.write().await;
        codes.insert(
            device_code.clone(),
            DeviceCodeEntry {
                user_code: user_code.clone(),
                username: None,
                scopes,
                expires_at: Instant::now() + Duration::from_secs(expires_in),
                approved: false,
            },
        );
    }

    let verify = format!("{host}/oauth/verify");
    let verify_complete = format!("{verify}?user_code={}", user_code);

    (
        StatusCode::OK,
        Json(DeviceCodeResp {
            device_code,
            user_code,
            verification_uri: verify,
            verification_uri_complete: verify_complete,
            expires_in,
            interval: 2,
        }),
    )
        .into_response()
}

#[derive(Debug, Deserialize)]
struct TokenReq {
    grant_type: String,
    device_code: String,
    client_id: String,
}

#[derive(Debug, Serialize)]
struct TokenResp {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
}

async fn oauth_token(State(state): State<AppState>, Json(req): Json<TokenReq>) -> Response {
    if req.grant_type != "urn:ietf:params:oauth:grant-type:device_code" {
        return oauth_err(StatusCode::BAD_REQUEST, "unsupported_grant_type");
    }

    let entry = {
        let codes = state.device_codes.read().await;
        codes.get(&req.device_code).cloned()
    };

    let Some(entry) = entry else {
        return oauth_err(StatusCode::BAD_REQUEST, "invalid_grant");
    };

    if Instant::now() > entry.expires_at {
        return oauth_err(StatusCode::BAD_REQUEST, "expired_token");
    }

    if !entry.approved {
        return oauth_err(StatusCode::BAD_REQUEST, "authorization_pending");
    }

    let Some(username) = entry.username else {
        return oauth_err(StatusCode::BAD_REQUEST, "invalid_grant");
    };

    let scope = if entry.scopes.is_empty() {
        "repo:read repo:write".to_string()
    } else {
        entry.scopes.join(" ")
    };

    let token = make_access_token(&state.auth, &username, &scope);

    {
        let mut codes = state.device_codes.write().await;
        codes.remove(&req.device_code);
    }

    let _ = req.client_id;

    (
        StatusCode::OK,
        Json(TokenResp {
            access_token: token,
            token_type: "Bearer".into(),
            expires_in: 3600,
            scope,
        }),
    )
        .into_response()
}

fn oauth_err(status: StatusCode, code: &'static str) -> Response {
    (status, Json(serde_json::json!({"error": code}))).into_response()
}

#[derive(Debug, Deserialize)]
struct VerifyQuery {
    user_code: Option<String>,
}

async fn oauth_verify_page(Query(query): Query<VerifyQuery>) -> Html<String> {
    let user_code = query.user_code.unwrap_or_default();
    Html(format!(
        "<html><body><h1>GX Device Verification</h1>
        <form method='post' action='/oauth/verify'>
        <label>User code <input name='user_code' value='{user_code}'/></label><br/>
        <label>Username <input name='username' value='alice'/></label><br/>
        <button type='submit'>Approve</button>
        </form></body></html>"
    ))
}

#[derive(Debug, Deserialize)]
struct VerifyForm {
    user_code: String,
    username: String,
}

async fn oauth_verify_submit(
    State(state): State<AppState>,
    Form(form): Form<VerifyForm>,
) -> Response {
    let mut codes = state.device_codes.write().await;
    if let Some((_, entry)) = codes
        .iter_mut()
        .find(|(_, entry)| entry.user_code.eq_ignore_ascii_case(&form.user_code))
    {
        if Instant::now() > entry.expires_at {
            return (StatusCode::BAD_REQUEST, Html(String::from("code expired"))).into_response();
        }
        entry.approved = true;
        entry.username = Some(form.username.clone());
        return (
            StatusCode::OK,
            Html(String::from("approved, return to gx cli")),
        )
            .into_response();
    }

    (StatusCode::NOT_FOUND, Html(String::from("unknown code"))).into_response()
}

#[derive(Debug, Deserialize)]
struct UiAuthQuery {
    access_token: Option<String>,
}

async fn ui_repos(
    State(state): State<AppState>,
    Query(query): Query<UiAuthQuery>,
    headers: HeaderMap,
) -> Response {
    let user = match authenticate(&state.auth, &headers, query.access_token.as_deref()) {
        Ok(u) => u,
        Err(resp) => return resp,
    };

    let mut rows = String::new();
    if let Ok(entries) = std::fs::read_dir(&state.repos_root) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".git") {
                rows.push_str(&format!(
                    "<li><a href='/ui/repos/{name}/tree/HEAD?access_token={}'>{name}</a></li>",
                    query.access_token.clone().unwrap_or_default()
                ));
            }
        }
    }

    Html(format!(
        "<html><body><h1>Repositories</h1><p>User: {}</p><ul>{}</ul></body></html>",
        user.username, rows
    ))
    .into_response()
}

async fn ui_tree(
    State(state): State<AppState>,
    AxumPath((repo, path)): AxumPath<(String, String)>,
    Query(query): Query<UiAuthQuery>,
    headers: HeaderMap,
) -> Response {
    let user = match authenticate(&state.auth, &headers, query.access_token.as_deref()) {
        Ok(u) => u,
        Err(resp) => return resp,
    };
    if !user.can_read() {
        return (StatusCode::FORBIDDEN, "missing repo:read").into_response();
    }

    let Some(repo_path) = safe_repo_path(&state.repos_root, &repo) else {
        return (StatusCode::BAD_REQUEST, "invalid repo").into_response();
    };
    let spec = if path == "HEAD" || path.is_empty() {
        "HEAD".to_string()
    } else {
        path
    };
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("ls-tree")
        .arg("--name-only")
        .arg(&spec)
        .output()
        .await;

    let Ok(output) = output else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "git ls-tree failed").into_response();
    };
    if !output.status.success() {
        return (
            StatusCode::BAD_REQUEST,
            String::from_utf8_lossy(&output.stderr).to_string(),
        )
            .into_response();
    }

    let token = query.access_token.unwrap_or_default();
    let mut list = String::new();
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        if line.is_empty() {
            continue;
        }
        list.push_str(&format!(
            "<li><a href='/ui/repos/{repo}/blob/{spec}/{line}?access_token={token}'>{line}</a></li>"
        ));
    }

    Html(format!(
        "<html><body><h1>{repo}:{spec}</h1><a href='/ui/repos?access_token={token}'>back</a><ul>{list}</ul></body></html>"
    ))
    .into_response()
}

async fn ui_blob(
    State(state): State<AppState>,
    AxumPath((repo, path)): AxumPath<(String, String)>,
    Query(query): Query<UiAuthQuery>,
    headers: HeaderMap,
) -> Response {
    let user = match authenticate(&state.auth, &headers, query.access_token.as_deref()) {
        Ok(u) => u,
        Err(resp) => return resp,
    };
    if !user.can_read() {
        return (StatusCode::FORBIDDEN, "missing repo:read").into_response();
    }

    let Some(repo_path) = safe_repo_path(&state.repos_root, &repo) else {
        return (StatusCode::BAD_REQUEST, "invalid repo").into_response();
    };
    let Some((rev, file_path)) = path.split_once('/') else {
        return (StatusCode::BAD_REQUEST, "need /blob/<rev>/<path>").into_response();
    };

    let obj = format!("{}:{}", rev, file_path);
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_path)
        .arg("show")
        .arg(&obj)
        .output()
        .await;

    let Ok(output) = output else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "git show failed").into_response();
    };

    if !output.status.success() {
        return (
            StatusCode::BAD_REQUEST,
            String::from_utf8_lossy(&output.stderr).to_string(),
        )
            .into_response();
    }

    let content = html_escape(&String::from_utf8_lossy(&output.stdout));
    Html(format!(
        "<html><body><h1>{obj}</h1><pre>{content}</pre></body></html>"
    ))
    .into_response()
}

#[derive(Debug, Clone)]
struct AuthUser {
    username: String,
    scopes: Vec<String>,
}

impl AuthUser {
    fn can_read(&self) -> bool {
        self.scopes
            .iter()
            .any(|s| s == "repo:read" || s == "repo:write")
    }
    fn can_write(&self) -> bool {
        self.scopes.iter().any(|s| s == "repo:write")
    }
}

async fn git_http_backend(
    State(state): State<AppState>,
    AxumPath(path): AxumPath<String>,
    method: Method,
    headers: HeaderMap,
    OriginalUri(uri): OriginalUri,
    body: Bytes,
) -> Response {
    let user = match authenticate(&state.auth, &headers, None) {
        Ok(user) => user,
        Err(resp) => return resp,
    };

    let op = classify_git_operation(uri.query().unwrap_or(""), &path, &method);
    if op.requires_write && !user.can_write() {
        return (StatusCode::FORBIDDEN, "missing repo:write").into_response();
    }
    if !op.requires_write && !user.can_read() {
        return (StatusCode::FORBIDDEN, "missing repo:read").into_response();
    }

    let path_info = format!("/{}", path.trim_start_matches('/'));
    let mut cmd = Command::new("git");
    cmd.arg("http-backend")
        .env("GIT_PROJECT_ROOT", &state.repos_root)
        .env("GIT_HTTP_EXPORT_ALL", "")
        .env("REQUEST_METHOD", method.as_str())
        .env("PATH_INFO", &path_info)
        .env("QUERY_STRING", uri.query().unwrap_or(""))
        .env("REMOTE_ADDR", "127.0.0.1")
        .env("REMOTE_USER", &user.username)
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

#[derive(Debug)]
struct GitOperation {
    requires_write: bool,
}

fn classify_git_operation(query: &str, path: &str, method: &Method) -> GitOperation {
    let is_receive_pack = query.contains("service=git-receive-pack")
        || path.ends_with("/git-receive-pack")
        || method == Method::from_bytes(b"POST").unwrap() && path.contains("receive-pack");
    GitOperation {
        requires_write: is_receive_pack,
    }
}

fn parse_scopes(scope: &str) -> Vec<String> {
    if scope.trim().is_empty() {
        vec!["repo:read".into(), "repo:write".into()]
    } else {
        scope
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    }
}

fn make_access_token(auth: &AuthConfig, username: &str, scope: &str) -> String {
    let now = Utc::now();
    let claims = Claims {
        sub: username.to_string(),
        aud: auth.audience.clone(),
        iss: auth.issuer.clone(),
        iat: now.timestamp() as usize,
        exp: (now + ChronoDuration::hours(1)).timestamp() as usize,
        scope: scope.to_string(),
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(auth.jwt_secret.as_bytes()),
    )
    .expect("failed to sign token")
}

fn authenticate(
    auth: &AuthConfig,
    headers: &HeaderMap,
    query_token: Option<&str>,
) -> Result<AuthUser, Response> {
    let token = extract_bearer(headers)
        .or_else(|| extract_basic_password(headers))
        .or(query_token.map(|s| s.to_string()));

    let Some(token) = token else {
        return Err(auth_required());
    };

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_aud = false;
    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(auth.jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|_| auth_required())?;

    if decoded.claims.iss != auth.issuer {
        return Err(auth_required());
    }
    if decoded.claims.aud != auth.audience {
        return Err(auth_required());
    }

    let scopes = decoded
        .claims
        .scope
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    Ok(AuthUser {
        username: decoded.claims.sub,
        scopes,
    })
}

fn extract_bearer(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(|v| v.to_string())
}

fn extract_basic_password(headers: &HeaderMap) -> Option<String> {
    let raw = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Basic "))?;

    let decoded = base64::engine::general_purpose::STANDARD.decode(raw).ok()?;
    let decoded = String::from_utf8(decoded).ok()?;
    let (_, password) = decoded.split_once(':')?;
    Some(password.to_string())
}

fn auth_required() -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::WWW_AUTHENTICATE,
        HeaderValue::from_static("Basic realm=\"git-http-rust\""),
    );
    (StatusCode::UNAUTHORIZED, headers, "authentication required").into_response()
}

fn host_base_url(headers: &HeaderMap) -> String {
    if let Some(host) = headers.get(header::HOST).and_then(|h| h.to_str().ok()) {
        format!("http://{host}")
    } else {
        "http://127.0.0.1:8080".to_string()
    }
}

fn safe_repo_path(root: &Path, repo: &str) -> Option<PathBuf> {
    if repo.contains("..") || repo.contains('\\') || repo.contains('/') {
        return None;
    }
    let path = root.join(repo);
    if path.exists() {
        Some(path)
    } else {
        None
    }
}

fn rand_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn html_escape(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

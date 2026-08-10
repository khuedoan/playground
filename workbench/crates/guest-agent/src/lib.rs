use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    process::Stdio,
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, Result};
use axum::{
    Json, Router,
    extract::{Path as AxumPath, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::Serialize;
use serde_json::Value;
use thiserror::Error;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines},
    process::{Child, ChildStdin, ChildStdout, Command},
    sync::Mutex,
    time::timeout,
};
use tracing::info;
use uuid::Uuid;
use workbench_protocol::{ExecRequest, ExecResponse, PiRpcRequest, PiRpcResponse};

#[derive(Debug, Error)]
pub enum GuestError {
    #[error("working directory must resolve inside {0}")]
    InvalidWorkingDirectory(String),
    #[error("invalid Pi session name")]
    InvalidSession,
    #[error("process timed out")]
    Timeout,
    #[error("Pi closed its RPC stream")]
    PiClosed,
    #[error("guest operation failed: {0}")]
    Operation(String),
}

struct PiProcess {
    child: Child,
    stdin: ChildStdin,
    stdout: Lines<BufReader<ChildStdout>>,
}

pub struct PiManager {
    executable: PathBuf,
    workspace_root: PathBuf,
    sessions: Mutex<HashMap<String, PiProcess>>,
    provider: Option<String>,
    model: Option<String>,
    api_key: Option<String>,
}

impl PiManager {
    pub fn new(
        executable: PathBuf,
        workspace_root: PathBuf,
        provider: Option<String>,
        model: Option<String>,
        api_key: Option<String>,
    ) -> Self {
        Self {
            executable,
            workspace_root,
            sessions: Mutex::new(HashMap::new()),
            provider,
            model,
            api_key,
        }
    }

    pub fn has_api_key(&self) -> bool {
        self.api_key.as_ref().is_some_and(|key| !key.is_empty())
    }

    async fn spawn(&self, session: &str) -> Result<PiProcess, GuestError> {
        validate_session(session)?;
        let session_dir = self.workspace_root.join(".pi/sessions");
        tokio::fs::create_dir_all(&session_dir)
            .await
            .map_err(|error| GuestError::Operation(error.to_string()))?;

        let mut command = Command::new(&self.executable);
        command
            .args([
                "--mode",
                "rpc",
                "--approve",
                "--name",
                session,
                "--session-dir",
            ])
            .arg(session_dir)
            .current_dir(&self.workspace_root)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true);
        if let Some(provider) = &self.provider {
            command.args(["--provider", provider]);
        }
        if let Some(model) = &self.model {
            command.args(["--model", model]);
        }
        if let Some(api_key) = &self.api_key {
            command.arg("--api-key").arg(api_key);
        }
        let mut child = command
            .spawn()
            .map_err(|error| GuestError::Operation(format!("start Pi: {error}")))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| GuestError::Operation("Pi stdin unavailable".to_owned()))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| GuestError::Operation("Pi stdout unavailable".to_owned()))?;
        Ok(PiProcess {
            child,
            stdin,
            stdout: BufReader::new(stdout).lines(),
        })
    }

    pub async fn rpc(
        &self,
        session: &str,
        mut request: Value,
    ) -> Result<PiRpcResponse, GuestError> {
        validate_session(session)?;
        let request_id = request
            .get("id")
            .and_then(Value::as_str)
            .map(str::to_owned)
            .unwrap_or_else(|| Uuid::new_v4().to_string());
        let object = request
            .as_object_mut()
            .ok_or_else(|| GuestError::Operation("Pi request must be a JSON object".to_owned()))?;
        object.insert("id".to_owned(), Value::String(request_id.clone()));

        let mut sessions = self.sessions.lock().await;
        if !sessions.contains_key(session) {
            let process = self.spawn(session).await?;
            sessions.insert(session.to_owned(), process);
        }
        let process = sessions.get_mut(session).expect("session was inserted");
        if process
            .child
            .try_wait()
            .map_err(|error| GuestError::Operation(error.to_string()))?
            .is_some()
        {
            sessions.remove(session);
            return Err(GuestError::PiClosed);
        }

        let mut bytes = serde_json::to_vec(&request)
            .map_err(|error| GuestError::Operation(error.to_string()))?;
        bytes.push(b'\n');
        process
            .stdin
            .write_all(&bytes)
            .await
            .map_err(|error| GuestError::Operation(error.to_string()))?;
        process
            .stdin
            .flush()
            .await
            .map_err(|error| GuestError::Operation(error.to_string()))?;

        let response = timeout(Duration::from_secs(30), async {
            let mut events = Vec::new();
            while let Some(line) = process
                .stdout
                .next_line()
                .await
                .map_err(|error| GuestError::Operation(error.to_string()))?
            {
                let value: Value = serde_json::from_str(&line)
                    .map_err(|error| GuestError::Operation(format!("invalid Pi JSONL: {error}")))?;
                let is_response = value.get("type").and_then(Value::as_str) == Some("response")
                    && value.get("id").and_then(Value::as_str) == Some(request_id.as_str());
                if is_response {
                    return Ok(PiRpcResponse {
                        response: value,
                        preceding_events: events,
                    });
                }
                events.push(value);
            }
            Err(GuestError::PiClosed)
        })
        .await
        .map_err(|_| GuestError::Timeout)??;
        Ok(response)
    }
}

fn validate_session(session: &str) -> Result<(), GuestError> {
    let valid = !session.is_empty()
        && session.len() <= 64
        && session
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_');
    if valid {
        Ok(())
    } else {
        Err(GuestError::InvalidSession)
    }
}

pub fn resolve_working_directory(root: &Path, requested: &Path) -> Result<PathBuf, GuestError> {
    let canonical_root = root
        .canonicalize()
        .map_err(|error| GuestError::Operation(error.to_string()))?;
    let canonical_requested = requested
        .canonicalize()
        .map_err(|_| GuestError::InvalidWorkingDirectory(root.display().to_string()))?;
    if canonical_requested.starts_with(&canonical_root) {
        Ok(canonical_requested)
    } else {
        Err(GuestError::InvalidWorkingDirectory(
            canonical_root.display().to_string(),
        ))
    }
}

#[derive(Clone)]
pub struct AppState {
    pub workspace_root: PathBuf,
    pub pi: Arc<PiManager>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route("/v1/exec", post(exec))
        .route("/v1/pi/sessions/{session}/rpc", post(pi_rpc))
        .with_state(state)
}

async fn health(State(state): State<AppState>) -> Json<Value> {
    Json(serde_json::json!({
        "status": "ok",
        "agent": "pi",
        "model_credentials": state.pi.has_api_key()
    }))
}

async fn exec(
    State(state): State<AppState>,
    Json(request): Json<ExecRequest>,
) -> Result<Json<ExecResponse>, ApiError> {
    let cwd = resolve_working_directory(&state.workspace_root, Path::new(&request.cwd))?;
    info!(command = %request.command, cwd = %cwd.display(), "executing guest command");
    let child = Command::new("bash")
        .args(["-lc", &request.command])
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .context("spawn guest command")
        .map_err(|error| GuestError::Operation(error.to_string()))?;

    let wait = timeout(
        Duration::from_secs(request.timeout_seconds),
        child.wait_with_output(),
    )
    .await;
    match wait {
        Ok(Ok(output)) => Ok(Json(ExecResponse {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            timed_out: false,
        })),
        Ok(Err(error)) => Err(GuestError::Operation(error.to_string()).into()),
        Err(_) => Ok(Json(ExecResponse {
            exit_code: None,
            stdout: String::new(),
            stderr: "command timed out".to_owned(),
            timed_out: true,
        })),
    }
}

async fn pi_rpc(
    State(state): State<AppState>,
    AxumPath(session): AxumPath<String>,
    Json(request): Json<PiRpcRequest>,
) -> Result<Json<PiRpcResponse>, ApiError> {
    state
        .pi
        .rpc(&session, request.request)
        .await
        .map(Json)
        .map_err(Into::into)
}

struct ApiError(GuestError);

impl From<GuestError> for ApiError {
    fn from(value: GuestError) -> Self {
        Self(value)
    }
}

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct Body<'a> {
            error: &'a str,
        }
        let status = match self.0 {
            GuestError::InvalidWorkingDirectory(_) | GuestError::InvalidSession => {
                StatusCode::BAD_REQUEST
            }
            GuestError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            GuestError::PiClosed | GuestError::Operation(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = self.0.to_string();
        axum::response::IntoResponse::into_response((status, Json(Body { error: &message })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn working_directory_cannot_escape_workspace() {
        let root = tempfile::tempdir().unwrap();
        let inside = root.path().join("inside");
        std::fs::create_dir(&inside).unwrap();
        assert_eq!(
            resolve_working_directory(root.path(), &inside).unwrap(),
            inside.canonicalize().unwrap()
        );
        assert!(resolve_working_directory(root.path(), Path::new("/")).is_err());
    }

    #[test]
    fn session_names_are_path_safe() {
        assert!(validate_session("thread-123").is_ok());
        assert!(validate_session("../escape").is_err());
        assert!(validate_session("spaces are not allowed").is_err());
    }
}

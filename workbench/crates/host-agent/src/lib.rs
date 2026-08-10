use std::{
    collections::{BTreeMap, HashMap},
    net::{Ipv4Addr, SocketAddrV4},
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, Result};
use async_trait::async_trait;
use axum::{
    Json, Router,
    extract::{Path as AxumPath, State},
    http::StatusCode,
    routing::get,
};
use chrono::Utc;
use serde::Serialize;
use thiserror::Error;
use tokio::{net::TcpStream, process::Command, sync::Mutex};
use tracing::info;
use uuid::Uuid;
use workbench_protocol::{ActualState, DesiredState, EnsureVmRequest, VmStatus};

const GUEST_AGENT_PORT: u16 = 7070;
const CODE_SERVER_PORT: u16 = 3000;
const NOVNC_PORT: u16 = 6080;

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("workspace id in the request does not match the URL")]
    WorkspaceMismatch,
    #[error("generation {received} is older than applied generation {applied}")]
    StaleGeneration { received: u64, applied: u64 },
    #[error("generation was reused with a different command")]
    GenerationConflict,
    #[error("backend failed: {0}")]
    Backend(String),
    #[error("state persistence failed: {0}")]
    Persistence(String),
}

#[async_trait]
pub trait VmBackend: Send + Sync + 'static {
    async fn apply(&self, request: &EnsureVmRequest) -> Result<VmStatus>;
}

pub struct MicrovmBackend {
    microvm: PathBuf,
    systemctl: PathBuf,
    flake_root: PathBuf,
    spec_root: PathBuf,
    state_root: PathBuf,
    health_timeout: Duration,
}

impl MicrovmBackend {
    pub fn new(
        microvm: PathBuf,
        systemctl: PathBuf,
        flake_root: PathBuf,
        spec_root: PathBuf,
        state_root: PathBuf,
        health_timeout: Duration,
    ) -> Self {
        Self {
            microvm,
            systemctl,
            flake_root,
            spec_root,
            state_root,
            health_timeout,
        }
    }

    fn vm_name(id: Uuid) -> String {
        format!("workbench-{id}")
    }

    fn tap_name(id: Uuid) -> String {
        let compact = id.simple().to_string();
        format!("wb-{}", &compact[..11])
    }

    fn network(id: Uuid) -> (Ipv4Addr, Ipv4Addr, String) {
        let bytes = id.as_bytes();
        let final_octet = u16::from(bytes[15]) % 253 + 2;
        let address = Ipv4Addr::new(10, 88, bytes[14], final_octet as u8);
        let gateway = Ipv4Addr::new(10, 88, 0, 1);
        let mac = format!(
            "02:b0:{:02x}:{:02x}:{:02x}:{:02x}",
            bytes[0], bytes[1], bytes[14], bytes[15]
        );
        (address, gateway, mac)
    }

    fn spec_dir(&self, name: &str) -> PathBuf {
        self.spec_root.join(name)
    }

    fn runner(&self, name: &str) -> PathBuf {
        self.state_root.join(name).join("current/bin/microvm-run")
    }

    fn unit(name: &str) -> String {
        format!("microvm@{name}.service")
    }

    fn render_spec(&self, request: &EnsureVmRequest) -> Result<String> {
        if !self.flake_root.is_absolute() {
            anyhow::bail!("WORKBENCH_FLAKE_ROOT must be an absolute path");
        }
        if !(1..=64).contains(&request.profile.vcpus) {
            anyhow::bail!("vcpus must be between 1 and 64");
        }
        if !(512..=131_072).contains(&request.profile.memory_mib) {
            anyhow::bail!("memory_mib must be between 512 and 131072");
        }
        if !(1..=2048).contains(&request.profile.disk_gib) {
            anyhow::bail!("disk_gib must be between 1 and 2048");
        }
        let name = Self::vm_name(request.workspace_id);
        let (address, gateway, mac) = Self::network(request.workspace_id);
        Ok(format!(
            r#"{{
  inputs.workbench.url = {flake_url};

  outputs = {{ self, workbench }}: {{
    nixosConfigurations.{name_attr} = workbench.lib.mkWorkspace {{
      workspaceName = {name};
      workspaceId = {workspace_id};
      vcpus = {vcpus};
      memoryMib = {memory_mib};
      diskGiB = {disk_gib};
      gui = {gui};
      address = {address};
      gateway = {gateway};
      mac = {mac};
      tapInterface = {tap};
    }};
  }};
}}
"#,
            flake_url = nix_string(&format!("path:{}", self.flake_root.display())),
            name_attr = nix_string(&name),
            name = nix_string(&name),
            workspace_id = nix_string(&request.workspace_id.to_string()),
            vcpus = request.profile.vcpus,
            memory_mib = request.profile.memory_mib,
            disk_gib = request.profile.disk_gib,
            gui = request.profile.gui,
            address = nix_string(&address.to_string()),
            gateway = nix_string(&gateway.to_string()),
            mac = nix_string(&mac),
            tap = nix_string(&Self::tap_name(request.workspace_id)),
        ))
    }

    async fn write_spec(&self, request: &EnsureVmRequest) -> Result<(PathBuf, bool)> {
        let name = Self::vm_name(request.workspace_id);
        let directory = self.spec_dir(&name);
        tokio::fs::create_dir_all(&directory)
            .await
            .with_context(|| format!("create {}", directory.display()))?;
        let path = directory.join("flake.nix");
        let contents = self.render_spec(request)?;
        let changed = match tokio::fs::read_to_string(&path).await {
            Ok(current) => current != contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => true,
            Err(error) => return Err(error).with_context(|| format!("read {}", path.display())),
        };
        if changed {
            let temporary = directory.join("flake.nix.tmp");
            tokio::fs::write(&temporary, contents)
                .await
                .with_context(|| format!("write {}", temporary.display()))?;
            tokio::fs::rename(&temporary, &path)
                .await
                .with_context(|| format!("replace {}", path.display()))?;
        }
        Ok((path, changed))
    }

    async fn run(&self, executable: &Path, args: &[String]) -> Result<String> {
        let output = Command::new(executable)
            .args(args)
            .output()
            .await
            .with_context(|| format!("invoke {}", executable.display()))?;
        if !output.status.success() {
            anyhow::bail!(
                "{} {}: {}",
                executable.display(),
                args.join(" "),
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }

    async fn is_active(&self, name: &str) -> Result<bool> {
        let status = Command::new(&self.systemctl)
            .args(["is-active", "--quiet", &Self::unit(name)])
            .status()
            .await
            .with_context(|| format!("invoke {}", self.systemctl.display()))?;
        Ok(status.success())
    }

    async fn stop(&self, name: &str) -> Result<()> {
        if self.is_active(name).await? {
            self.run(&self.systemctl, &["stop".to_owned(), Self::unit(name)])
                .await?;
        }
        Ok(())
    }

    async fn remove_tree(path: &Path) -> Result<()> {
        match tokio::fs::remove_dir_all(path).await {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error).with_context(|| format!("remove {}", path.display())),
        }
    }

    async fn wait_healthy(&self, address: Ipv4Addr) -> Result<()> {
        if self.health_timeout.is_zero() {
            return Ok(());
        }
        let deadline = tokio::time::Instant::now() + self.health_timeout;
        let socket = SocketAddrV4::new(address, GUEST_AGENT_PORT);
        while tokio::time::Instant::now() < deadline {
            if matches!(
                tokio::time::timeout(Duration::from_secs(1), TcpStream::connect(socket)).await,
                Ok(Ok(_))
            ) {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
        anyhow::bail!("guest agent at {socket} did not become ready")
    }

    fn running_status(request: &EnsureVmRequest, address: Ipv4Addr) -> VmStatus {
        VmStatus {
            workspace_id: request.workspace_id,
            generation: request.generation,
            command_id: request.command_id,
            desired_state: request.desired_state,
            actual_state: ActualState::Running,
            ip_address: Some(address.to_string()),
            desktop_url: Some(format!(
                "http://{address}:{NOVNC_PORT}/vnc.html?autoconnect=1&resize=scale"
            )),
            code_url: Some(format!("http://{address}:{CODE_SERVER_PORT}")),
            agent_url: Some(format!("http://{address}:{GUEST_AGENT_PORT}")),
            error: None,
            updated_at: Utc::now(),
        }
    }
}

#[async_trait]
impl VmBackend for MicrovmBackend {
    async fn apply(&self, request: &EnsureVmRequest) -> Result<VmStatus> {
        let name = Self::vm_name(request.workspace_id);
        let (address, _, _) = Self::network(request.workspace_id);
        match request.desired_state {
            DesiredState::Running => {
                let (spec, changed) = self.write_spec(request).await?;
                let runner_exists = tokio::fs::try_exists(self.runner(&name)).await?;
                if !runner_exists {
                    self.run(
                        &self.microvm,
                        &[
                            "-f".to_owned(),
                            spec.display().to_string(),
                            "-c".to_owned(),
                            name.clone(),
                        ],
                    )
                    .await?;
                } else if changed {
                    self.stop(&name).await?;
                    self.run(&self.microvm, &["-u".to_owned(), name.clone()])
                        .await?;
                }
                if !self.is_active(&name).await? {
                    self.run(&self.systemctl, &["start".to_owned(), Self::unit(&name)])
                        .await?;
                }
                self.wait_healthy(address).await?;
                Ok(Self::running_status(request, address))
            }
            DesiredState::Stopped => {
                self.stop(&name).await?;
                Ok(status_for(request, ActualState::Stopped))
            }
            DesiredState::Deleted => {
                self.stop(&name).await?;
                Self::remove_tree(&self.state_root.join(&name)).await?;
                Self::remove_tree(&self.spec_dir(&name)).await?;
                Ok(status_for(request, ActualState::Deleted))
            }
        }
    }
}

fn nix_string(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace("${", "\\${")
            .replace('"', "\\\"")
    )
}

fn status_for(request: &EnsureVmRequest, actual_state: ActualState) -> VmStatus {
    VmStatus {
        workspace_id: request.workspace_id,
        generation: request.generation,
        command_id: request.command_id,
        desired_state: request.desired_state,
        actual_state,
        ip_address: None,
        desktop_url: None,
        code_url: None,
        agent_url: None,
        error: None,
        updated_at: Utc::now(),
    }
}

pub struct VmStore {
    path: PathBuf,
    records: Mutex<BTreeMap<Uuid, VmStatus>>,
    workspace_locks: Mutex<HashMap<Uuid, Arc<Mutex<()>>>>,
    backend: Arc<dyn VmBackend>,
}

impl VmStore {
    pub async fn open(path: PathBuf, backend: Arc<dyn VmBackend>) -> Result<Self> {
        let records = match tokio::fs::read(&path).await {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .with_context(|| format!("invalid state file {}", path.display()))?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => BTreeMap::new(),
            Err(error) => return Err(error).with_context(|| format!("read {}", path.display())),
        };
        Ok(Self {
            path,
            records: Mutex::new(records),
            workspace_locks: Mutex::new(HashMap::new()),
            backend,
        })
    }

    pub async fn get(&self, id: Uuid) -> Option<VmStatus> {
        self.records.lock().await.get(&id).cloned()
    }

    pub async fn list(&self) -> Vec<VmStatus> {
        self.records.lock().await.values().cloned().collect()
    }

    pub async fn ensure(
        &self,
        path_id: Uuid,
        request: EnsureVmRequest,
    ) -> Result<VmStatus, StoreError> {
        if path_id != request.workspace_id {
            return Err(StoreError::WorkspaceMismatch);
        }

        let workspace_lock = {
            let mut locks = self.workspace_locks.lock().await;
            locks
                .entry(path_id)
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone()
        };
        let _workspace_guard = workspace_lock.lock().await;

        {
            let records = self.records.lock().await;
            if let Some(applied) = records.get(&path_id) {
                if applied.command_id == request.command_id {
                    return Ok(applied.clone());
                }
                if request.generation < applied.generation {
                    return Err(StoreError::StaleGeneration {
                        received: request.generation,
                        applied: applied.generation,
                    });
                }
                if request.generation == applied.generation {
                    return Err(StoreError::GenerationConflict);
                }
            }
        }

        let status = self
            .backend
            .apply(&request)
            .await
            .map_err(|error| StoreError::Backend(error.to_string()))?;

        let mut records = self.records.lock().await;
        records.insert(path_id, status.clone());
        self.persist(&records).await?;
        Ok(status)
    }

    async fn persist(&self, records: &BTreeMap<Uuid, VmStatus>) -> Result<(), StoreError> {
        if let Some(parent) = self.path.parent() {
            tokio::fs::create_dir_all(parent)
                .await
                .map_err(|error| StoreError::Persistence(error.to_string()))?;
        }
        let temporary = self.path.with_extension("json.tmp");
        let bytes = serde_json::to_vec_pretty(records)
            .map_err(|error| StoreError::Persistence(error.to_string()))?;
        tokio::fs::write(&temporary, bytes)
            .await
            .map_err(|error| StoreError::Persistence(error.to_string()))?;
        tokio::fs::rename(&temporary, &self.path)
            .await
            .map_err(|error| StoreError::Persistence(error.to_string()))?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<VmStore>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route("/v1/workspaces", get(list_workspaces))
        .route(
            "/v1/workspaces/{workspace_id}",
            get(get_workspace).put(ensure_workspace),
        )
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok", "backend": "microvm.nix"}))
}

async fn list_workspaces(State(state): State<AppState>) -> Json<Vec<VmStatus>> {
    Json(state.store.list().await)
}

async fn get_workspace(
    State(state): State<AppState>,
    AxumPath(workspace_id): AxumPath<Uuid>,
) -> Result<Json<VmStatus>, StatusCode> {
    state
        .store
        .get(workspace_id)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn ensure_workspace(
    State(state): State<AppState>,
    AxumPath(workspace_id): AxumPath<Uuid>,
    Json(request): Json<EnsureVmRequest>,
) -> Result<Json<VmStatus>, ApiError> {
    info!(%workspace_id, generation = request.generation, "applying desired state");
    state
        .store
        .ensure(workspace_id, request)
        .await
        .map(Json)
        .map_err(ApiError)
}

struct ApiError(StoreError);

impl axum::response::IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        #[derive(Serialize)]
        struct Body<'a> {
            error: &'a str,
        }
        let status = match self.0 {
            StoreError::WorkspaceMismatch
            | StoreError::StaleGeneration { .. }
            | StoreError::GenerationConflict => StatusCode::CONFLICT,
            StoreError::Backend(_) | StoreError::Persistence(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        let message = self.0.to_string();
        axum::response::IntoResponse::into_response((status, Json(Body { error: &message })))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use workbench_protocol::VmProfile;

    #[derive(Default)]
    struct MockBackend;

    #[async_trait]
    impl VmBackend for MockBackend {
        async fn apply(&self, request: &EnsureVmRequest) -> Result<VmStatus> {
            let actual_state = match request.desired_state {
                DesiredState::Running => ActualState::Running,
                DesiredState::Stopped => ActualState::Stopped,
                DesiredState::Deleted => ActualState::Deleted,
            };
            Ok(status_for(request, actual_state))
        }
    }

    fn request(id: Uuid, command_id: Uuid, generation: u64) -> EnsureVmRequest {
        EnsureVmRequest {
            command_id,
            workspace_id: id,
            generation,
            desired_state: DesiredState::Running,
            profile: VmProfile::default(),
        }
    }

    fn executable(path: &Path, script: String) {
        std::fs::write(path, script).unwrap();
        let mut permissions = std::fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions).unwrap();
    }

    #[tokio::test]
    async fn command_is_idempotent_and_state_survives_restart() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("state.json");
        let id = Uuid::new_v4();
        let command_id = Uuid::new_v4();
        let store = VmStore::open(path.clone(), Arc::new(MockBackend))
            .await
            .unwrap();

        let first = store.ensure(id, request(id, command_id, 1)).await.unwrap();
        let second = store.ensure(id, request(id, command_id, 1)).await.unwrap();
        assert_eq!(first.command_id, second.command_id);

        let reopened = VmStore::open(path, Arc::new(MockBackend)).await.unwrap();
        assert_eq!(reopened.get(id).await.unwrap().generation, 1);
    }

    #[tokio::test]
    async fn stale_generation_is_rejected() {
        let directory = tempfile::tempdir().unwrap();
        let store = VmStore::open(directory.path().join("state.json"), Arc::new(MockBackend))
            .await
            .unwrap();
        let id = Uuid::new_v4();
        store
            .ensure(id, request(id, Uuid::new_v4(), 2))
            .await
            .unwrap();
        assert!(matches!(
            store.ensure(id, request(id, Uuid::new_v4(), 1)).await,
            Err(StoreError::StaleGeneration { .. })
        ));
    }

    struct CountingBackend(AtomicUsize);

    #[async_trait]
    impl VmBackend for CountingBackend {
        async fn apply(&self, request: &EnsureVmRequest) -> Result<VmStatus> {
            self.0.fetch_add(1, Ordering::SeqCst);
            tokio::time::sleep(Duration::from_millis(25)).await;
            MockBackend.apply(request).await
        }
    }

    #[tokio::test]
    async fn concurrent_retries_only_apply_backend_once() {
        let directory = tempfile::tempdir().unwrap();
        let backend = Arc::new(CountingBackend(AtomicUsize::new(0)));
        let store = Arc::new(
            VmStore::open(directory.path().join("state.json"), backend.clone())
                .await
                .unwrap(),
        );
        let workspace_id = Uuid::new_v4();
        let command_id = Uuid::new_v4();

        let first = tokio::spawn({
            let store = store.clone();
            async move {
                store
                    .ensure(workspace_id, request(workspace_id, command_id, 1))
                    .await
            }
        });
        let second = tokio::spawn({
            let store = store.clone();
            async move {
                store
                    .ensure(workspace_id, request(workspace_id, command_id, 1))
                    .await
            }
        });

        first.await.unwrap().unwrap();
        second.await.unwrap().unwrap();
        assert_eq!(backend.0.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn microvm_backend_generates_and_starts_a_resource_limited_guest() {
        let directory = tempfile::tempdir().unwrap();
        let microvm = directory.path().join("microvm");
        let systemctl = directory.path().join("systemctl");
        let command_log = directory.path().join("commands.log");
        let active = directory.path().join("active");
        let state_root = directory.path().join("microvms");
        let flake_root = directory.path().join("source");
        std::fs::create_dir(&flake_root).unwrap();

        executable(
            &microvm,
            format!(
                "#!/bin/sh\necho microvm \"$@\" >> {log}\nif [ \"$1\" = -f ]; then mkdir -p {state}/$4/current/bin; touch {state}/$4/current/bin/microvm-run; fi\n",
                log = command_log.display(),
                state = state_root.display()
            ),
        );
        executable(
            &systemctl,
            format!(
                "#!/bin/sh\necho systemctl \"$@\" >> {log}\ncase \"$1\" in is-active) test -f {active};; start) touch {active};; stop) rm -f {active};; esac\n",
                log = command_log.display(),
                active = active.display()
            ),
        );

        let backend = MicrovmBackend::new(
            microvm,
            systemctl,
            flake_root,
            directory.path().join("specs"),
            state_root,
            Duration::ZERO,
        );
        let id = Uuid::new_v4();
        let request = request(id, Uuid::new_v4(), 1);
        let status = backend.apply(&request).await.unwrap();

        assert_eq!(status.actual_state, ActualState::Running);
        assert_eq!(
            status
                .ip_address
                .as_deref()
                .unwrap()
                .split('.')
                .take(2)
                .collect::<Vec<_>>(),
            ["10", "88"]
        );
        assert!(
            status
                .desktop_url
                .as_deref()
                .unwrap()
                .contains(":6080/vnc.html")
        );
        assert!(status.code_url.as_deref().unwrap().contains(":3000"));
        let name = MicrovmBackend::vm_name(id);
        let spec =
            std::fs::read_to_string(directory.path().join("specs").join(&name).join("flake.nix"))
                .unwrap();
        assert!(spec.contains("workbench.lib.mkWorkspace"));
        assert!(spec.contains("memoryMib = 8192"));
        assert!(spec.contains("diskGiB = 40"));
        let calls = std::fs::read_to_string(command_log).unwrap();
        assert!(calls.contains("microvm -f"));
        assert!(calls.contains(&format!("systemctl start microvm@{name}.service")));
    }
}

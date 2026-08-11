use std::{net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};

use anyhow::Result;
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use workbench_host_agent::{AppState, MicrovmBackend, MicrovmBackendConfig, VmStore, router};
use workbench_protocol::VmProfile;

#[derive(Debug, Parser)]
struct Options {
    #[arg(long, env = "WORKBENCH_HOST_LISTEN", default_value = "127.0.0.1:9090")]
    listen: SocketAddr,
    #[arg(
        long,
        env = "WORKBENCH_HOST_STATE",
        default_value = "/var/lib/workbench-host/state.json"
    )]
    state: PathBuf,
    #[arg(long, env = "WORKBENCH_MICROVM", default_value = "microvm")]
    microvm: PathBuf,
    #[arg(long, env = "WORKBENCH_SYSTEMCTL", default_value = "systemctl")]
    systemctl: PathBuf,
    #[arg(long, env = "WORKBENCH_FLAKE_ROOT")]
    flake_root: PathBuf,
    #[arg(
        long,
        env = "WORKBENCH_SPEC_ROOT",
        default_value = "/var/lib/workbench/specs"
    )]
    spec_root: PathBuf,
    #[arg(
        long,
        env = "WORKBENCH_MICROVM_STATE_ROOT",
        default_value = "/var/lib/microvms"
    )]
    microvm_state_root: PathBuf,
    #[arg(
        long,
        env = "WORKBENCH_POOL_STATE",
        default_value = "/var/lib/workbench/pool.json"
    )]
    pool_state: PathBuf,
    #[arg(
        long,
        env = "WORKBENCH_GUEST_HEALTH_TIMEOUT_SECONDS",
        default_value_t = 180
    )]
    guest_health_timeout_seconds: u64,
    #[arg(long, env = "WORKBENCH_E2E_MOCK_LLM", default_value_t = false)]
    e2e_mock_llm: bool,
    #[arg(long, env = "WORKBENCH_WARM_POOL_SIZE", default_value_t = 3)]
    warm_pool_size: u16,
    #[arg(long, env = "WORKBENCH_POOL_VCPUS", default_value_t = 4)]
    pool_vcpus: u16,
    #[arg(long, env = "WORKBENCH_POOL_MEMORY_MIB", default_value_t = 8192)]
    pool_memory_mib: u32,
    #[arg(long, env = "WORKBENCH_POOL_DISK_GIB", default_value_t = 40)]
    pool_disk_gib: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let options = Options::parse();
    let backend = Arc::new(MicrovmBackend::new(MicrovmBackendConfig {
        microvm: options.microvm,
        systemctl: options.systemctl,
        flake_root: options.flake_root,
        spec_root: options.spec_root,
        state_root: options.microvm_state_root,
        pool_state_path: options.pool_state,
        health_timeout: Duration::from_secs(options.guest_health_timeout_seconds),
        e2e_mock_llm: options.e2e_mock_llm,
        pool_size: options.warm_pool_size,
        pool_profile: VmProfile {
            vcpus: options.pool_vcpus,
            memory_mib: options.pool_memory_mib,
            disk_gib: options.pool_disk_gib,
            gui: true,
        },
    })?);
    backend.warm_pool().await?;
    let store = Arc::new(VmStore::open(options.state, backend).await?);
    let app = router(AppState { store }).layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind(options.listen).await?;
    info!(address = %options.listen, backend = "microvm.nix", "host agent listening");
    axum::serve(listener, app).await?;
    Ok(())
}

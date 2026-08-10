use std::{net::SocketAddr, path::PathBuf, sync::Arc, time::Duration};

use anyhow::Result;
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use workbench_host_agent::{AppState, MicrovmBackend, VmStore, router};

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
        env = "WORKBENCH_CREDENTIAL_ROOT",
        default_value = "/run/workbench/credentials"
    )]
    credential_root: PathBuf,
    #[arg(
        long,
        env = "WORKBENCH_GUEST_HEALTH_TIMEOUT_SECONDS",
        default_value_t = 180
    )]
    guest_health_timeout_seconds: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let options = Options::parse();
    let backend = Arc::new(MicrovmBackend::new(
        options.microvm,
        options.systemctl,
        options.flake_root,
        options.spec_root,
        options.microvm_state_root,
        options.credential_root,
        Duration::from_secs(options.guest_health_timeout_seconds),
    ));
    let store = Arc::new(VmStore::open(options.state, backend).await?);
    let app = router(AppState { store }).layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind(options.listen).await?;
    info!(address = %options.listen, backend = "microvm.nix", "host agent listening");
    axum::serve(listener, app).await?;
    Ok(())
}

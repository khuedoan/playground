use std::{net::SocketAddr, path::PathBuf, sync::Arc};

use anyhow::Result;
use clap::Parser;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use workbench_guest_agent::{AppState, PiManager, router};

#[derive(Debug, Parser)]
struct Options {
    #[arg(long, env = "WORKBENCH_GUEST_LISTEN", default_value = "0.0.0.0:7070")]
    listen: SocketAddr,
    #[arg(long, env = "WORKBENCH_WORKSPACE_ROOT", default_value = "/workspace")]
    workspace_root: PathBuf,
    #[arg(long, env = "PI_EXECUTABLE", default_value = "pi")]
    pi_executable: PathBuf,
    #[arg(long, env = "PI_PROVIDER")]
    pi_provider: Option<String>,
    #[arg(long, env = "PI_MODEL")]
    pi_model: Option<String>,
    #[arg(long, env = "PI_API_KEY")]
    pi_api_key: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    let options = Options::parse();
    let pi_api_key = options
        .pi_api_key
        .or_else(|| match options.pi_provider.as_deref() {
            Some("github-models") => std::env::var("GITHUB_MODELS_TOKEN").ok(),
            _ => None,
        });
    let pi = Arc::new(PiManager::new(
        options.pi_executable,
        options.workspace_root.clone(),
        options.pi_provider,
        options.pi_model,
        pi_api_key,
    ));
    let app = router(AppState {
        workspace_root: options.workspace_root,
        pi,
    })
    .layer(TraceLayer::new_for_http());
    let listener = TcpListener::bind(options.listen).await?;
    info!(address = %options.listen, "guest agent listening");
    axum::serve(listener, app).await?;
    Ok(())
}

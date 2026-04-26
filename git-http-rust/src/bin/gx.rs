use std::{fs, path::PathBuf, time::Duration};

use clap::{Parser, Subcommand};
use dirs::config_dir;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;

#[derive(Debug, Parser)]
#[command(name = "gx", about = "Git eXperience CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    Credential {
        #[command(subcommand)]
        command: CredentialCommands,
    },
}

#[derive(Debug, Subcommand)]
enum AuthCommands {
    Login {
        #[arg(long)]
        server: String,
        #[arg(long, default_value = "gx")]
        client_id: String,
        #[arg(long, default_value = "repo:read repo:write")]
        scope: String,
        #[arg(long, default_value = "alice")]
        username: String,
    },
    Token,
}

#[derive(Debug, Subcommand)]
enum CredentialCommands {
    Fill,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct StoredCreds {
    server: String,
    access_token: String,
}

#[derive(Debug, Deserialize)]
struct DeviceCodeResp {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: String,
    interval: u64,
}

#[derive(Debug, Deserialize)]
struct TokenResp {
    access_token: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Auth { command } => match command {
            AuthCommands::Login {
                server,
                client_id,
                scope,
                username,
            } => {
                if let Err(e) = auth_login(&server, &client_id, &scope, &username).await {
                    eprintln!("login failed: {e}");
                    std::process::exit(1);
                }
            }
            AuthCommands::Token => {
                let creds = load_creds().unwrap_or_default();
                println!("{}", creds.access_token);
            }
        },
        Commands::Credential { command } => match command {
            CredentialCommands::Fill => {
                let creds = load_creds().unwrap_or_default();
                println!("username=oauth2");
                println!("password={}", creds.access_token);
            }
        },
    }
}

async fn auth_login(
    server: &str,
    client_id: &str,
    scope: &str,
    username: &str,
) -> anyhow::Result<()> {
    let client = Client::builder().no_proxy().build()?;

    let device = client
        .post(format!("{server}/oauth/device/code"))
        .json(&serde_json::json!({"client_id": client_id, "scope": scope}))
        .send()
        .await?
        .error_for_status()?
        .json::<DeviceCodeResp>()
        .await?;

    println!("Open in browser: {}", device.verification_uri_complete);
    println!("Verification page: {}", device.verification_uri);

    // Dev automation: auto-approve so e2e can run non-interactively.
    client
        .post(format!("{server}/oauth/verify"))
        .form(&[
            ("user_code", device.user_code.as_str()),
            ("username", username),
        ])
        .send()
        .await?
        .error_for_status()?;

    loop {
        let res = client
            .post(format!("{server}/oauth/token"))
            .json(&serde_json::json!({
                "grant_type": "urn:ietf:params:oauth:grant-type:device_code",
                "device_code": device.device_code,
                "client_id": client_id,
            }))
            .send()
            .await?;

        if res.status().is_success() {
            let token = res.json::<TokenResp>().await?;
            save_creds(&StoredCreds {
                server: server.to_string(),
                access_token: token.access_token,
            })?;
            println!("gx login successful");
            return Ok(());
        }

        let err: serde_json::Value = res.json().await.unwrap_or_default();
        if err.get("error").and_then(|v| v.as_str()) == Some("authorization_pending") {
            sleep(Duration::from_secs(device.interval.max(1))).await;
            continue;
        }

        anyhow::bail!("token error: {err}");
    }
}

fn creds_path() -> PathBuf {
    config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("gx")
        .join("credentials.json")
}

fn save_creds(creds: &StoredCreds) -> anyhow::Result<()> {
    let path = creds_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, serde_json::to_vec_pretty(creds)?)?;
    Ok(())
}

fn load_creds() -> anyhow::Result<StoredCreds> {
    let path = creds_path();
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

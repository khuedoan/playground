mod agent;
mod config;
mod llm;
mod sandbox;
mod tools;

use std::io::{self, BufRead, Write};
use std::path::PathBuf;

use clap::Parser;

use crate::agent::Agent;
use crate::config::Config;

/// openclaw-rs — a minimal, secure coding agent CLI
#[derive(Parser, Debug)]
#[command(version, about)]
struct Cli {
    /// Path to configuration file (default: openclaw.toml)
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Override the LLM model
    #[arg(short, long)]
    model: Option<String>,

    /// Run a single prompt (non-interactive)
    #[arg(short, long)]
    prompt: Option<String>,

    /// Sandbox root directory (default: current directory)
    #[arg(long)]
    root: Option<PathBuf>,

    /// Disable approval prompts for writes
    #[arg(long)]
    no_approve: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut config = Config::load(cli.config.as_deref());

    // Apply CLI overrides
    if let Some(model) = cli.model {
        config.provider.model = model;
    }
    if let Some(root) = cli.root {
        config.sandbox.root = root;
    }
    if cli.no_approve {
        config.sandbox.require_approval = false;
    }

    eprintln!(
        "openclaw-rs v{} | model={} | root={}",
        env!("CARGO_PKG_VERSION"),
        config.provider.model,
        config.sandbox.root.display(),
    );

    let mut agent = Agent::new(&config);

    // Single-shot mode
    if let Some(prompt) = cli.prompt {
        match agent.process(&prompt).await {
            Ok(_reply) => {} // Already printed via streaming
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
        return;
    }

    // Interactive REPL
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    loop {
        eprint!("\n> ");
        io::stderr().flush().ok();

        let line = match reader.next() {
            Some(Ok(line)) => line,
            _ => break,
        };

        let input = line.trim();
        if input.is_empty() {
            continue;
        }
        if input == "/quit" || input == "/exit" {
            break;
        }
        if input == "/help" {
            eprintln!("Commands: /quit /exit /help");
            eprintln!("Type a message to chat with the coding agent.");
            continue;
        }

        match agent.process(input).await {
            Ok(_reply) => {} // Already printed via streaming
            Err(e) => eprintln!("error: {e}"),
        }
    }
}


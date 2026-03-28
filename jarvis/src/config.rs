use std::path::{Path, PathBuf};

use serde::Deserialize;

/// Top-level configuration loaded from `jarvis.toml`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Config {
    pub provider: ProviderConfig,
    pub sandbox: SandboxConfig,
    pub agent: AgentConfig,
}

/// LLM provider settings.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ProviderConfig {
    /// Base URL for the OpenAI-compatible API.
    pub api_base: String,
    /// Model identifier (e.g. "gpt-4o", "claude-sonnet-4-20250514").
    pub model: String,
    /// Maximum tokens in the response.
    pub max_tokens: u32,
    /// Sampling temperature.
    pub temperature: f32,
}

/// Sandbox / security policy.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct SandboxConfig {
    /// Root directory the agent is allowed to access.
    /// Defaults to the current working directory.
    pub root: PathBuf,
    /// Shell commands the agent may execute without approval.
    pub allowed_commands: Vec<String>,
    /// Whether destructive file writes require interactive approval.
    pub require_approval: bool,
}

/// Agent behaviour knobs.
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct AgentConfig {
    /// System prompt prepended to every conversation.
    pub system_prompt: String,
    /// Maximum tool-call rounds per user message.
    pub max_rounds: usize,
}

// ── Defaults ────────────────────────────────────────────────────────────

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            api_base: "https://api.openai.com/v1".into(),
            model: "gpt-4o".into(),
            max_tokens: 4096,
            temperature: 0.0,
        }
    }
}

impl Default for SandboxConfig {
    fn default() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            allowed_commands: vec![
                "ls".into(),
                "cat".into(),
                "head".into(),
                "tail".into(),
                "grep".into(),
                "find".into(),
                "wc".into(),
                "diff".into(),
                "git".into(),
                "cargo".into(),
                "make".into(),
                "echo".into(),
            ],
            require_approval: true,
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            system_prompt: concat!(
                "You are a coding assistant. You can read files, write files, ",
                "run shell commands, and search codebases. Be concise and precise. ",
                "Always verify your changes compile before finishing.",
            )
            .into(),
            max_rounds: 20,
        }
    }
}

// ── Loading ─────────────────────────────────────────────────────────────

impl Config {
    /// Try to load configuration from a file, falling back to defaults.
    pub fn load(path: Option<&Path>) -> Self {
        let candidates: Vec<PathBuf> = if let Some(p) = path {
            vec![p.to_path_buf()]
        } else {
            vec![
                PathBuf::from("jarvis.toml"),
                dirs::config_dir()
                    .map(|d| d.join("jarvis").join("config.toml"))
                    .unwrap_or_default(),
            ]
        };

        for candidate in &candidates {
            if let Ok(contents) = std::fs::read_to_string(candidate) {
                match toml::from_str::<Config>(&contents) {
                    Ok(cfg) => return cfg,
                    Err(e) => {
                        eprintln!("warning: failed to parse {}: {e}", candidate.display());
                    }
                }
            }
        }

        Config::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_valid() {
        let cfg = Config::default();
        assert_eq!(cfg.provider.model, "gpt-4o");
        assert!(cfg.sandbox.require_approval);
        assert!(!cfg.sandbox.allowed_commands.is_empty());
        assert!(cfg.agent.max_rounds > 0);
    }

    #[test]
    fn parse_minimal_toml() {
        let toml_str = r#"
[provider]
model = "claude-sonnet-4-20250514"

[sandbox]
require_approval = false
"#;
        let cfg: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(cfg.provider.model, "claude-sonnet-4-20250514");
        assert!(!cfg.sandbox.require_approval);
        // Defaults should still apply for unset fields
        assert_eq!(cfg.provider.max_tokens, 4096);
    }

    #[test]
    fn load_returns_defaults_when_no_file() {
        let cfg = Config::load(Some(Path::new("/nonexistent/path.toml")));
        assert_eq!(cfg.provider.model, "gpt-4o");
    }
}

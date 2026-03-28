# openclaw-rs — Research Notes

## Objective

Research and build a minimal, secure, open-source coding agent CLI in Rust,
inspired by OpenClaw, OpenAI Codex CLI, and Pi.

## Key Inspirations

### OpenClaw
- Personal AI assistant running on your own devices
- Local-first gateway architecture
- Multi-channel support (Telegram, Slack, Discord, etc.)
- Skills system with 5,400+ community skills
- DM pairing and allowlist security model

### Codex CLI (OpenAI)
- Config-driven approach with `config.toml`
- MCP server integration for tool extensibility
- Profile system for different workflows
- Slash commands and custom prompts

### Pi (badlogic/pi-mono)
- Advanced tool system: LSP integration, git, Python execution
- Streaming terminal UI with real-time feedback
- Task orchestration with subagent support
- Code review tool with structured findings
- Conventional commit generation

## Design Decisions

### Why Rust?
- Memory safety without garbage collection
- Single static binary distribution
- Strong type system catches bugs at compile time
- Excellent async ecosystem (tokio)
- Sandboxing primitives (seccomp, landlock on Linux)

### Architecture
- **Single binary**: No runtime dependencies beyond the OS
- **Config-driven**: TOML configuration for providers, tools, and policies
- **Secure by default**: Sandboxed command execution with explicit allowlists
- **Streaming**: Token-by-token output for responsive UX
- **Tool system**: Pluggable tools for file I/O, shell, and search
- **Provider-agnostic**: Support multiple LLM backends via a unified trait

### Security Model
- Directory-scoped file access (no escaping the project root)
- Command allowlist for shell execution
- Network isolation options for sandboxed runs
- Configurable approval flow for destructive operations

## Metrics
- **Primary**: Binary size (lower is better, target < 10 MB release)
- **Secondary**: Cold start latency (lower is better, target < 100ms)
- **Tertiary**: Lines of code (lower is better for maintainability)

## Log

### 2026-03-28 — Initial research and scaffolding
- Surveyed OpenClaw, Codex CLI, and Pi architectures
- Chose Rust for safety, performance, and single-binary distribution
- Designed minimal module layout: config, llm, tools, sandbox, cli
- Implemented core scaffolding with streaming LLM client
- Implemented sandbox with directory scoping and command allowlists
- Implemented tool system: read_file, write_file, shell, search

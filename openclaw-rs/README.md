# openclaw-rs

A minimal, secure coding agent CLI written in Rust. Research experiment
exploring what an open-source alternative to tools like
[OpenClaw](https://github.com/openclaw/openclaw) could look like, with
design inspiration from [Codex CLI](https://github.com/openai/codex) and
[Pi](https://github.com/badlogic/pi-mono).

## Question

What does a minimal yet secure coding agent look like when built from
scratch in Rust? What are the essential building blocks, and how small can
the implementation be while remaining useful?

## Design

### Architecture

```
┌────────────┐
│   CLI/REPL │  clap + tokio
├────────────┤
│   Agent    │  conversation loop with tool dispatch
├────────────┤
│  LLM Client│  OpenAI-compatible streaming (SSE)
├────────────┤
│   Tools    │  read_file, write_file, shell, search
├────────────┤
│  Sandbox   │  path confinement + command allowlist
└────────────┘
```

### Key Decisions

| Decision | Rationale |
|----------|-----------|
| **Rust** | Memory safety, single binary, strong types |
| **OpenAI-compatible API** | Works with OpenAI, Anthropic (via proxy), local models |
| **Streaming SSE** | Token-by-token output for responsive UX |
| **Path confinement** | All file ops scoped to a root directory, traversal blocked |
| **Command allowlist** | Only pre-approved binaries can be executed |
| **TOML config** | Simple, human-readable, supports provider/sandbox/agent sections |
| **Minimal dependencies** | clap, tokio, reqwest, serde — nothing exotic |

### Security Model (inspired by OpenClaw + Codex)

- **Directory sandbox**: File reads/writes are confined to a configurable
  root directory. Path traversal (`../`) is detected and rejected.
- **Command allowlist**: Only commands listed in `allowed_commands` can
  execute. Unknown binaries are blocked by default.
- **Approval flow**: Destructive operations (file writes) can require
  interactive user confirmation (`require_approval = true`).
- **No ambient credentials**: The API key is read from an environment
  variable, never stored on disk by the tool.

### Tool System (inspired by Pi)

| Tool | Description |
|------|-------------|
| `read_file` | Read file contents within the sandbox |
| `write_file` | Create or overwrite a file (with optional approval) |
| `shell` | Execute an allowlisted command and return output |
| `search` | Recursive grep for a pattern across the project |

## Setup

### Prerequisites

- Rust toolchain (1.70+)
- An OpenAI-compatible API key

### Build

```sh
cargo build --release
```

### Configure (optional)

Create `openclaw.toml` in your project root:

```toml
[provider]
api_base = "https://api.openai.com/v1"
model = "gpt-4o"
max_tokens = 4096

[sandbox]
allowed_commands = ["ls", "cat", "grep", "git", "cargo", "make"]
require_approval = true

[agent]
system_prompt = "You are a coding assistant."
max_rounds = 20
```

### Run

```sh
# Set your API key
export OPENAI_API_KEY="sk-..."

# Interactive REPL
cargo run

# Single prompt
cargo run -- --prompt "Explain this codebase"

# Override model
cargo run -- --model claude-sonnet-4-20250514

# Disable write approval
cargo run -- --no-approve
```

## Results

- **20 tests** covering config parsing, sandbox path confinement, command
  allowlisting, tool dispatch, and serialization.
- **~600 lines of Rust** across 6 modules.
- Clean separation of concerns: config → sandbox → tools → LLM → agent → CLI.
- The sandbox successfully blocks path traversal and disallowed commands.

## Reproduction

```sh
cd openclaw-rs
cargo test
cargo build --release
ls -lh target/release/openclaw-rs
```

## What's Next

Potential extensions (not in scope for this experiment):

- [ ] MCP server integration for tool extensibility (à la Codex CLI)
- [ ] LSP integration for code intelligence (à la Pi)
- [ ] Multi-provider support with native Anthropic/Google APIs
- [ ] Linux sandboxing via Landlock/seccomp
- [ ] Conversation persistence and session resume
- [ ] TUI with syntax-highlighted diffs

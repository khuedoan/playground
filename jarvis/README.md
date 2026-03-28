# jarvis

A minimal, secure coding agent CLI written in Rust, shipped as a
Nix-built Docker image. Research experiment exploring what the smallest
useful coding agent looks like, with design inspiration from
[Codex CLI](https://github.com/openai/codex) and
[Pi](https://github.com/badlogic/pi-mono).

## Question

What does a minimal yet secure coding agent look like when built from
scratch in Rust? How do we address the
["lethal trifecta"](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
— access to private data, ability to execute code, and exposure to
untrusted content — while keeping the implementation small?

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
├────────────┤
│  Container │  Nix-built Docker image (isolation layer)
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
| **Nix-built Docker image** | Reproducible, minimal, no distro baggage |
| **Minimal dependencies** | clap, tokio, reqwest, serde — nothing exotic |

### Security Model — Addressing the Lethal Trifecta

Simon Willison's ["lethal trifecta"](https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/)
identifies the dangerous combination present in coding agents:

1. **Access to private data** — the agent reads your source code
2. **Ability to execute code** — the agent runs shell commands
3. **Exposure to untrusted content** — files, dependencies, LLM outputs

When all three are present, prompt injection attacks become possible:
malicious content in a file or dependency can trick the agent into
exfiltrating data or running destructive commands.

Jarvis mitigates each leg of the trifecta:

| Trifecta Leg | Mitigation |
|--------------|------------|
| **Private data access** | Directory sandbox confines reads/writes to a single root. Path traversal (`../`) is detected and rejected. |
| **Code execution** | Command allowlist — only pre-approved binaries can run. No arbitrary shell. |
| **Untrusted content** | Docker container provides OS-level isolation. Network can be disabled (`--network=none`). Approval flow for writes. |

Additional defenses:
- **No ambient credentials**: API key via env var only, never on disk.
- **Container isolation**: The Nix-built Docker image runs with minimal
  tools, no package manager, no compiler. Use `--network=none` to fully
  block exfiltration.
- **Approval flow**: Destructive writes can require interactive
  confirmation (`require_approval = true`).

### Tool System

| Tool | Description |
|------|-------------|
| `read_file` | Read file contents within the sandbox |
| `write_file` | Create or overwrite a file (with optional approval) |
| `shell` | Execute an allowlisted command and return output |
| `search` | Recursive grep for a pattern across the project |

## Setup

### Prerequisites

- Rust toolchain (1.70+), or Nix, or Docker
- An OpenAI-compatible API key

### Build (cargo)

```sh
cargo build --release
```

### Build (Nix)

```sh
# First build will fail with a hash mismatch — copy the correct hash
# from the error message into flake.nix, then build again.
nix build .#jarvis
```

### Build Docker image (Nix)

```sh
nix build .#dockerImages.x86_64-linux.default
docker load < result
```

### Configure (optional)

Create `jarvis.toml` in your project root:

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

### Run (native)

```sh
export OPENAI_API_KEY="sk-..."

# Interactive REPL
jarvis

# Single prompt
jarvis --prompt "Explain this codebase"

# Override model
jarvis --model claude-sonnet-4-20250514

# Disable write approval
jarvis --no-approve
```

### Run (Docker — recommended for untrusted repos)

```sh
# Full isolation: no network, read-only source mount
docker run --rm -it \
  --network=none \
  -v "$(pwd)":/work:ro \
  -e OPENAI_API_KEY \
  jarvis:latest --root /work --no-approve

# Allow network (needed for LLM API calls in practice)
docker run --rm -it \
  -v "$(pwd)":/work \
  -e OPENAI_API_KEY \
  jarvis:latest --root /work
```

## Results

- **20 tests** covering config parsing, sandbox path confinement, command
  allowlisting, tool dispatch, and serialization.
- **~1,200 lines of Rust** across 6 modules (including tests).
- Clean separation of concerns: config → sandbox → tools → LLM → agent → CLI.
- The sandbox successfully blocks path traversal and disallowed commands.
- Nix flake builds the binary and produces a Docker image with no distro
  baggage — just the binary, coreutils, grep, git, and CA certs.

## Reproduction

```sh
cd jarvis

# Test
cargo test

# Build
cargo build --release
ls -lh target/release/jarvis

# Docker image (requires Nix on Linux)
nix build .#dockerImages.x86_64-linux.default
docker load < result
docker run --rm jarvis:latest --help
```

## What's Next

Potential extensions (not in scope for this experiment):

- [ ] MCP server integration for tool extensibility (à la Codex CLI)
- [ ] LSP integration for code intelligence (à la Pi)
- [ ] Multi-provider support with native Anthropic/Google APIs
- [ ] Linux sandboxing via Landlock/seccomp (defense in depth inside container)
- [ ] Conversation persistence and session resume
- [ ] TUI with syntax-highlighted diffs
- [ ] Network egress firewall — allow only the LLM API endpoint


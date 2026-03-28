# jarvis — Research Notes

## Objective

Research and build a minimal, secure, open-source coding agent CLI in Rust,
shipped as a Nix-built Docker image. Inspired by Codex CLI and Pi.

## Key Inspirations

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

### The Lethal Trifecta (Simon Willison)
- https://simonwillison.net/2025/Jun/16/the-lethal-trifecta/
- Coding agents combine three dangerous capabilities:
  1. Access to private data (source code, secrets)
  2. Ability to execute code (shell commands)
  3. Exposure to untrusted content (files, deps, LLM output)
- This makes prompt injection attacks especially dangerous
- Mitigations: sandbox file access, allowlist commands, container isolation,
  network restriction (`--network=none`), approval flow for writes

## Design Decisions

### Why Rust?
- Memory safety without garbage collection
- Single static binary distribution
- Strong type system catches bugs at compile time
- Excellent async ecosystem (tokio)
- Sandboxing primitives (seccomp, landlock on Linux)

### Why Nix-built Docker?
- Reproducible builds — bit-for-bit identical images
- Minimal image — no distro, no package manager, no shell bloat
- Just the binary + coreutils + grep + git + CA certs
- Easy to audit: `nix build` from source, `docker load`, done

### Architecture
- **Single binary**: No runtime dependencies beyond the OS
- **Config-driven**: TOML configuration for providers, tools, and policies
- **Secure by default**: Sandboxed command execution with explicit allowlists
- **Streaming**: Token-by-token output for responsive UX
- **Tool system**: Pluggable tools for file I/O, shell, and search
- **Provider-agnostic**: Support multiple LLM backends via a unified trait
- **Container-first**: Ship as a Docker image for OS-level isolation

### Security Model (addressing the lethal trifecta)
- Directory-scoped file access (no escaping the project root)
- Command allowlist for shell execution
- Docker container isolation (run with `--network=none` for full lockdown)
- Configurable approval flow for destructive operations
- No ambient credentials — API key via env var only

## Metrics
- **Primary**: Binary size (lower is better, target < 10 MB release)
- **Secondary**: Cold start latency (lower is better, target < 100ms)
- **Tertiary**: Lines of code (lower is better for maintainability)

## Log

### 2026-03-28 — Initial research and scaffolding
- Surveyed Codex CLI and Pi architectures
- Chose Rust for safety, performance, and single-binary distribution
- Designed minimal module layout: config, llm, tools, sandbox, cli
- Implemented core scaffolding with streaming LLM client
- Implemented sandbox with directory scoping and command allowlists
- Implemented tool system: read_file, write_file, shell, search

### 2026-03-28 — Rename, Docker image, lethal trifecta
- Renamed from openclaw-rs to jarvis
- Added Nix flake outputs for building the package and Docker image
- Documented Simon Willison's lethal trifecta and how jarvis mitigates it
- Docker image includes only: binary, coreutils, grep, find, diff, git, CA certs
- Recommended `--network=none` for untrusted repos

### 2026-03-28 — Dockerfile, Makefile, practical completion
- Added standalone `Dockerfile` using `nixos/nix` base (works with just Docker)
- Multi-stage build: NixOS builder stage + NixOS runtime stage
- Runtime image includes only sandboxed tools: coreutils, grep, find, diff, git
- Added `.dockerignore` to keep build context clean
- Added `Makefile` with targets: build, test, run, docker-build, docker-run, docker-run-isolated
- Updated README with both Docker build paths (Dockerfile vs pure Nix)
- All 20 tests still pass


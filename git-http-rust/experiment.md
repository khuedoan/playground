# Experiment session

## Objective
Ship a minimal Git Smart HTTP server in Rust that supports `git clone`, `git pull`, and `git push` with no authentication.

## Constraints
- Use Rust with Tokio + Axum.
- Keep this iteration small and understandable.
- No auth in this version.
- Keep all files scoped to this experiment directory.

## Files in scope
- `Cargo.toml`
- `src/main.rs`
- `README.md`
- `notes.md`
- `experiment.md`
- `flake.nix`
- `.envrc`
- `Makefile`
- `scripts/e2e.sh`

## Stop conditions
- Server compiles.
- Health endpoint works.
- Git CLI can clone, push, and pull successfully against the server.

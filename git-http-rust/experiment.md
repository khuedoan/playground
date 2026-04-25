# Experiment session

## Objective
Ship a minimal Git Smart HTTP server in Rust that supports clone/fetch (`upload-pack`) with no authentication.

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

## Stop conditions
- Server compiles.
- Basic health endpoint works.
- Smart HTTP upload-pack paths are wired and documented.

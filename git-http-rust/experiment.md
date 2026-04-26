# Experiment session

## Objective
Deliver phase-B style auth now: OAuth2 Device Flow + `gx` CLI, while preserving Git HTTP clone/push/pull and adding minimal repository browsing UI.

## Constraints
- Use Rust, Tokio, Axum.
- Keep auth and git gateway in this experiment.
- Keep no external IdP dependency for local e2e reproducibility.

## Files in scope
- `Cargo.toml`
- `src/main.rs`
- `src/bin/gx.rs`
- `scripts/e2e.sh`
- `README.md`
- `notes.md`
- `experiment.md`
- `Makefile`

## Stop conditions
- `gx auth login` obtains token via device flow.
- token-authenticated git clone/push/pull works.
- `/ui/repos` displays repository list with token.
- e2e script passes.

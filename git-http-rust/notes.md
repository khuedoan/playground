# Notes

## Objective
Build a minimal self-hosted Git Smart HTTP server in Rust (no auth) as the first milestone toward a combined Git + PaaS platform.

## Primary metric
- **Metric:** local clone success rate
- **Unit:** successful clone attempts / total attempts
- **Target direction:** higher is better (aim 100%)

## Baseline
- No existing server in this experiment directory.
- Baseline clone success rate: 0/1 (nothing listening).

## Iteration log
- Created Axum + Tokio server with routes for:
  - `GET /healthz`
  - `GET /:repo/info/refs?service=git-upload-pack`
  - `POST /:repo/git-upload-pack`
- Implemented the Git protocol handling by invoking:
  - `git upload-pack --stateless-rpc --advertise-refs <repo>` for ref advertisement.
  - `git upload-pack --stateless-rpc <repo>` for RPC upload-pack requests.
- Scoped repositories to a configurable root (`--repos-root`) and a single path segment repo name for safety in v1.

## Results
- Server compiles and test suite passes (`cargo test`).
- Local health check succeeded.
- This establishes the first end-to-end Git HTTP read path foundation.

## Decisions
- Use `git` process execution first (simpler, protocol-correct bootstrap) instead of implementing pkt-line parsing from scratch.
- Keep no-auth behavior explicit for v1.
- Defer push (`receive-pack`) and authentication/authorization to the next milestone.

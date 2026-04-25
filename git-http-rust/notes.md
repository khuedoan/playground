# Notes

## Objective
Build a minimal self-hosted Git Smart HTTP server in Rust (no auth) as the first milestone toward a combined Git + PaaS platform.

## Primary metric
- **Metric:** end-to-end Git operation success rate
- **Unit:** successful `clone + push + pull` runs / total runs
- **Target direction:** higher is better (aim 100%)

## Baseline
- Initial attempt only wired `upload-pack` manually.
- It did not satisfy full CLI workflow requirements, especially push.

## Iteration log
- Replaced manual upload-pack wiring with a generic CGI bridge to `git http-backend`.
- Added one catch-all Git route (`/*path`) to pass Git HTTP requests through to the backend.
- Implemented CGI response parsing (`Status` + headers + body) into Axum `Response`.
- Passed request method, path, query string, content type, and content length to `git http-backend`.
- Added reproducible e2e script target that verifies:
  - clone
  - commit + push
  - second clone + pull
- Added detailed README architecture walkthrough explaining request routing, CGI env mapping, backend process handling, and response parsing.

## Results
- `cargo fmt`, `cargo test`, and `cargo build` pass.
- e2e Git CLI workflow succeeds locally with HTTP 1:1 Git commands.
- Current metric from the scripted run: 1/1 successful `clone + push + pull`.

## Decisions
- Keep the no-auth behavior explicit for v1.
- Prefer `git http-backend` instead of implementing Smart HTTP packet handling manually.
- Defer auth, org/repo permissions, and PaaS-specific deployment flow to next iterations.

# git-http-rust

Self-hosted Git Smart HTTP prototype in Rust with:
- Git transport via `git http-backend`
- OAuth2 Device Flow auth (for a `gx auth login` UX)
- Basic web repo/content viewer

## What works in this version
- `git clone`, `git pull`, `git push` over HTTP with auth
- OAuth2 Device Flow endpoints:
  - `POST /oauth/device/code`
  - `POST /oauth/token`
  - `GET|POST /oauth/verify`
- `gx` CLI for login/token retrieval
- Web UI:
  - `GET /ui/repos`
  - `GET /ui/repos/:repo/tree/*path`
  - `GET /ui/repos/:repo/blob/*path`

## Architecture summary

1. Axum handles all git HTTP paths with `/*path`.
2. Before calling Git backend, server authenticates JWT from:
   - `Authorization: Bearer <token>`
   - or Basic auth password (`oauth2:<token>`)
   - or `access_token` query for UI pages.
3. Server checks scopes:
   - `repo:read` for read ops
   - `repo:write` for receive-pack/push
4. If authorized, server executes `git http-backend` and maps CGI output to HTTP response.
5. Device flow (`/oauth/device/code` + `/oauth/verify` + `/oauth/token`) mints HS256 JWT access tokens used by Git/UI.

## gx CLI usage

```bash
# login via device flow
cargo run --bin gx -- auth login --server http://127.0.0.1:8080 --username alice

# get token
TOKEN=$(cargo run --quiet --bin gx -- auth token)

# clone with token as basic password
git clone "http://oauth2:${TOKEN}@127.0.0.1:8080/demo.git"
```

## Run

```bash
make run
```

## End-to-end check

```bash
make e2e
```

This validates:
- OAuth2 device login with `gx`
- authenticated clone/push/pull
- authenticated web repo listing

## Dev notes
- Push requires `http.receivepack=true` on bare repos.
- JWT signing is local HS256 in this prototype.
- For production, replace with external OIDC provider + JWKS verification.

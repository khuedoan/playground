# git-http-rust

Minimal **Git Smart HTTP** server in Rust for the first milestone of a self-hosted Git + PaaS platform.

## Question
Can we stand up a simple, no-auth Git HTTP server in Rust that is compatible with basic clone/fetch flows?

## Setup
- Runtime: Tokio
- HTTP framework: Axum
- Git protocol handling: delegates to `git upload-pack --stateless-rpc`
- Repository storage: local bare repos under a configurable directory (`--repos-root`)

## What this version supports
- `GET /healthz`
- `GET /:repo/info/refs?service=git-upload-pack`
- `POST /:repo/git-upload-pack`

This is enough for the read side of Smart HTTP (clone/fetch) for a repository name like `demo.git` located in `repos/demo.git`.

## What this version does not support (yet)
- Authentication / authorization
- Push (`git-receive-pack`)
- Multi-segment repo paths (`org/repo.git`)
- Web UI or PaaS deployment features

## Run

```bash
make run
```

Default bind is `127.0.0.1:8080` and repos root is `./repos`.

## Quick local demo

```bash
# 1) Create a bare repository under repos/
mkdir -p repos
git init --bare repos/demo.git

# 2) Run the server
cargo run -- --listen 127.0.0.1:8080 --repos-root ./repos

# 3) In another terminal, clone via Smart HTTP
git clone http://127.0.0.1:8080/demo.git
```

## Reproduce checks

```bash
make test
curl -i http://127.0.0.1:8080/healthz
```

## Primary metric
- Clone success rate (successful clone attempts / total attempts), where higher is better.

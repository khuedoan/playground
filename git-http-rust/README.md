# git-http-rust

Minimal **Git Smart HTTP** server in Rust for the first milestone of a self-hosted Git + PaaS platform.

## Question
Can we stand up a simple, no-auth Git HTTP server in Rust that works with standard `git clone`, `git pull`, and `git push`?

## Setup
- Runtime: Tokio
- HTTP framework: Axum
- Git protocol handling: delegates to `git http-backend` (CGI bridge)
- Repository storage: local bare repos under a configurable directory (`--repos-root`)

## What this version supports
- `GET /healthz`
- Git Smart HTTP read + write via `git http-backend`:
  - upload-pack (clone/fetch/pull)
  - receive-pack (push)

## What this version does not support (yet)
- Authentication / authorization
- Multi-tenant access controls
- Web UI or PaaS deployment features

## Run

```bash
make run
```

Default bind is `127.0.0.1:8080` and repos root is `./repos`.

## Quick local demo (clone + push + pull)

```bash
# 1) Start clean
rm -rf repos tmp
mkdir -p repos tmp

# 2) Create remote bare repository
git init --bare repos/demo.git
git -C repos/demo.git config http.receivepack true
git -C repos/demo.git symbolic-ref HEAD refs/heads/main

# 3) Run server
cargo run -- --listen 127.0.0.1:8080 --repos-root ./repos

# 4) In another terminal: clone, commit, push, and pull
cd tmp
git clone http://127.0.0.1:8080/demo.git writer
cd writer
git config user.name "Dev"
git config user.email "dev@example.com"
echo "hello" > README.md
git add README.md
git commit -m "init"
git push origin HEAD:main

cd ..
git clone http://127.0.0.1:8080/demo.git reader
cd reader
git pull --ff-only origin main
```

## Reproduce checks

```bash
make test
make e2e
```

## Primary metric
- End-to-end Git operation success rate (`clone + push + pull` successful runs / total runs), where higher is better.

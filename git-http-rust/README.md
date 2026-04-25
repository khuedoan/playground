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

## Detailed: how the server works

This server is intentionally thin. It does **not** reimplement the Git protocol itself.
Instead, it translates incoming HTTP requests into the CGI-style interface expected by `git http-backend`, executes `git http-backend`, then translates the CGI output back into an HTTP response.

### 1) Incoming request routing

- Axum serves:
  - `/healthz` for liveness checks.
  - `/*path` for all Git HTTP requests.
- The catch-all route means requests like these are all handled:
  - `/demo.git/info/refs?service=git-upload-pack`
  - `/demo.git/git-upload-pack`
  - `/demo.git/info/refs?service=git-receive-pack`
  - `/demo.git/git-receive-pack`

### 2) CGI env mapping for `git http-backend`

For each request on `/*path`, the server spawns:

```bash
git http-backend
```

Then it sets environment variables expected by Git’s CGI interface:

- `GIT_PROJECT_ROOT=<repos root>`
  - Root folder containing bare repositories.
- `GIT_HTTP_EXPORT_ALL=`
  - Enables exporting repos without requiring `git-daemon-export-ok`.
- `REQUEST_METHOD=<GET|POST|...>`
- `PATH_INFO=/<route path>`
  - Example: `/demo.git/info/refs`
- `QUERY_STRING=<raw query string>`
  - Example: `service=git-upload-pack`
- `CONTENT_TYPE=<request content-type>` (if present)
- `CONTENT_LENGTH=<request body length>` (if non-empty)
- `REMOTE_ADDR=127.0.0.1`

Request body bytes are piped directly to the child process stdin.

### 3) Child process response parsing

`git http-backend` writes CGI output to stdout, typically like:

- CGI headers (`Status`, `Content-Type`, cache headers, etc.)
- blank line
- response body bytes (pkt-line payload or other Git HTTP content)

The server parses stdout by:

1. Finding header/body boundary (`\r\n\r\n` or `\n\n`).
2. Parsing header lines.
3. Converting `Status: NNN ...` to an HTTP status code.
4. Forwarding remaining headers as HTTP headers.
5. Returning body bytes unchanged.

If parsing fails, it returns HTTP 500.

### 4) Why clone/pull/push all work

Because `git http-backend` is the official Git-side HTTP implementation:

- `git clone` and `git pull` use upload-pack endpoints.
- `git push` uses receive-pack endpoints.
- Capability negotiation and pkt-line details are handled by Git itself.

This avoids fragile, partial protocol reimplementation in Rust for v1.

### 5) Important push prerequisite (bare repo)

For pushes over HTTP to a bare repository, enable:

```bash
git -C repos/<name>.git config http.receivepack true
```

Without it, pushes may return 403.

### 6) Known behavior and limitations

- No auth: anyone with network access can read/write to exposed repos.
- Trust boundary: this is a local/dev-first milestone, not production hardened.
- Process model: one `git http-backend` child per request.
- Observability is basic (stdout logging + process errors).

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

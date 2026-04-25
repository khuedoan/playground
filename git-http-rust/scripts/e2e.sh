#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PORT="${PORT:-18090}"
BASE="http://127.0.0.1:${PORT}"
BIN="${ROOT_DIR}/target/debug/git-http-rust"

cleanup() {
  if [[ -n "${SERVER_PID:-}" ]]; then
    kill "${SERVER_PID}" 2>/dev/null || true
    wait "${SERVER_PID}" 2>/dev/null || true
  fi
}
trap cleanup EXIT

cd "${ROOT_DIR}"

cargo build >/dev/null

rm -rf repos tmp-e2e
mkdir -p repos tmp-e2e

git init --bare repos/demo.git >/dev/null
git -C repos/demo.git config http.receivepack true
git -C repos/demo.git symbolic-ref HEAD refs/heads/main

"${BIN}" --listen "127.0.0.1:${PORT}" --repos-root "${ROOT_DIR}/repos" >/tmp/git-http-rust-e2e.log 2>&1 &
SERVER_PID=$!
sleep 1

# writer clone + commit + push
cd tmp-e2e
git clone "${BASE}/demo.git" writer >/dev/null
cd writer
git config user.name "E2E Dev"
git config user.email "e2e@example.com"
echo "hello from e2e" > README.md
git add README.md
git commit -m "init" >/dev/null
git push origin HEAD:main >/dev/null
cd ..

# reader clone + pull

git clone "${BASE}/demo.git" reader >/dev/null
cd reader
git pull --ff-only origin main >/dev/null
grep -q "hello from e2e" README.md

echo "e2e ok: clone + push + pull"

#!/usr/bin/env bash
set -euo pipefail

# Simple local bootstrap for OpenBao in dev mode.
# Usage:
#   ./scripts/bootstrap.sh
#
# Requirements:
#   - OpenBao CLI in PATH (`bao` or `openbao`) or BAO_BIN set.
#   - jq in PATH

resolve_bao_bin() {
  if [[ -n "${BAO_BIN:-}" ]] && [[ -x "${BAO_BIN}" ]]; then
    echo "$BAO_BIN"
    return 0
  fi

  if command -v bao >/dev/null 2>&1; then
    command -v bao
    return 0
  fi

  if command -v openbao >/dev/null 2>&1; then
    command -v openbao
    return 0
  fi

  return 1
}

if ! BAO_BIN="$(resolve_bao_bin)"; then
  echo "OpenBao CLI not found. Install it or run ./scripts/install-openbao.sh"
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq not found in PATH. Install jq before continuing."
  exit 1
fi

export BAO_ADDR="${BAO_ADDR:-http://127.0.0.1:8200}"
export BAO_DEV_ROOT_TOKEN_ID="${BAO_DEV_ROOT_TOKEN_ID:-dev-only-root-token}"

if pgrep -f "${BAO_BIN} server -dev" >/dev/null 2>&1; then
  echo "OpenBao dev server already running."
else
  echo "Starting OpenBao dev server with ${BAO_BIN}..."
  nohup "$BAO_BIN" server -dev -dev-root-token-id="$BAO_DEV_ROOT_TOKEN_ID" \
    > .openbao-dev.log 2>&1 &
  sleep 2
fi

export BAO_TOKEN="$BAO_DEV_ROOT_TOKEN_ID"

echo "Checking OpenBao status..."
"$BAO_BIN" status >/dev/null

# Create a sample kv-v2 secret.
"$BAO_BIN" secrets enable -path=apps kv-v2 >/dev/null 2>&1 || true
"$BAO_BIN" kv put apps/payments/api \
  username="payments-service" \
  password="replace-me" \
  endpoint="https://api.example.internal" >/dev/null

echo "Wrote secret to: apps/data/payments/api"

echo "\nCurrent secret (redact before sharing):"
"$BAO_BIN" kv get -format=json apps/payments/api \
  | jq '.data.data | with_entries(if .key == "password" then .value = "***REDACTED***" else . end)'

echo "\nBootstrap complete."
echo "BAO_BIN=$BAO_BIN"
echo "BAO_ADDR=$BAO_ADDR"
echo "BAO_TOKEN=$BAO_TOKEN"

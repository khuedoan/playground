#!/usr/bin/env bash
set -euo pipefail

# Simple local bootstrap for OpenBao in dev mode.
# Usage:
#   ./scripts/bootstrap.sh
#
# Requirements:
#   - openbao CLI in PATH
#   - jq in PATH

if ! command -v openbao >/dev/null 2>&1; then
  echo "openbao CLI not found in PATH. Enter dev shell first: nix develop"
  exit 1
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq not found in PATH. Enter dev shell first: nix develop"
  exit 1
fi

export BAO_ADDR="http://127.0.0.1:8200"
export BAO_DEV_ROOT_TOKEN_ID="dev-only-root-token"

if pgrep -f "openbao server -dev" >/dev/null 2>&1; then
  echo "OpenBao dev server already running."
else
  echo "Starting OpenBao dev server..."
  nohup openbao server -dev -dev-root-token-id="$BAO_DEV_ROOT_TOKEN_ID" \
    > .openbao-dev.log 2>&1 &
  sleep 2
fi

export BAO_TOKEN="$BAO_DEV_ROOT_TOKEN_ID"

echo "Checking OpenBao status..."
openbao status >/dev/null

# Create a sample kv-v2 secret.
openbao secrets enable -path=apps kv-v2 >/dev/null 2>&1 || true
openbao kv put apps/payments/api \
  username="payments-service" \
  password="replace-me" \
  endpoint="https://api.example.internal" >/dev/null

echo "Wrote secret to: apps/data/payments/api"

echo "\nCurrent secret (redact before sharing):"
openbao kv get -format=json apps/payments/api \
  | jq '.data.data | with_entries(if .key == "password" then .value = "***REDACTED***" else . end)'

echo "\nBootstrap complete."
echo "BAO_ADDR=$BAO_ADDR"
echo "BAO_TOKEN=$BAO_TOKEN"

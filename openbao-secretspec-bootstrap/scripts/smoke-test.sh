#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BAO_BIN_DEFAULT="$ROOT_DIR/.tools/bin/bao"
export BAO_BIN="${BAO_BIN:-$BAO_BIN_DEFAULT}"
export BAO_ADDR="${BAO_ADDR:-http://127.0.0.1:8200}"
export BAO_DEV_ROOT_TOKEN_ID="${BAO_DEV_ROOT_TOKEN_ID:-dev-only-root-token}"
export BAO_TOKEN="$BAO_DEV_ROOT_TOKEN_ID"

if [[ ! -x "$BAO_BIN" ]]; then
  echo "Missing BAO binary at $BAO_BIN"
  echo "Run: ./scripts/install-openbao.sh"
  exit 1
fi

"$ROOT_DIR/scripts/bootstrap.sh"

USERNAME="$("$BAO_BIN" kv get -field=username apps/payments/api)"
ENDPOINT="$("$BAO_BIN" kv get -field=endpoint apps/payments/api)"

if [[ "$USERNAME" != "payments-service" ]]; then
  echo "Unexpected username: $USERNAME"
  exit 1
fi

if [[ "$ENDPOINT" != "https://api.example.internal" ]]; then
  echo "Unexpected endpoint: $ENDPOINT"
  exit 1
fi

echo "Smoke test passed."

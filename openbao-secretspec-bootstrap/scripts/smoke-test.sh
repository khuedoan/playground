#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BAO_BIN_DEFAULT="$ROOT_DIR/.tools/bin/bao"
export BAO_BIN="${BAO_BIN:-$BAO_BIN_DEFAULT}"
export BAO_ADDR="${BAO_ADDR:-http://127.0.0.1:8200}"
export BAO_DEV_ROOT_TOKEN_ID="${BAO_DEV_ROOT_TOKEN_ID:-dev-only-root-token}"
export BAO_TOKEN="${BAO_TOKEN:-$BAO_DEV_ROOT_TOKEN_ID}"
export SECRETS_FILE="${SECRETS_FILE:-$ROOT_DIR/bootstrap-secrets.json}"

# Non-interactive values that would normally be pasted from third parties.
export PAYMENTS_ENDPOINT="${PAYMENTS_ENDPOINT:-https://api.example.internal}"
export STRIPE_API_KEY="${STRIPE_API_KEY:-sk_test_example_key}"
export STRIPE_WEBHOOK_SECRET="${STRIPE_WEBHOOK_SECRET:-whsec_example_secret}"

if [[ ! -x "$BAO_BIN" ]]; then
  echo "Missing BAO binary at $BAO_BIN"
  echo "Run: ./scripts/install-openbao.sh"
  exit 1
fi

"$ROOT_DIR/scripts/bootstrap.sh"

USERNAME="$($BAO_BIN kv get -field=username apps/payments/api)"
ENDPOINT="$($BAO_BIN kv get -field=endpoint apps/payments/api)"
STRIPE_KEY="$($BAO_BIN kv get -field=api_key apps/thirdparty/stripe)"
ROTATION_ID="$($BAO_BIN kv get -field=rotation_id apps/thirdparty/stripe)"

if [[ "$USERNAME" != "payments-service" ]]; then
  echo "Unexpected username: $USERNAME"
  exit 1
fi

if [[ "$ENDPOINT" != "$PAYMENTS_ENDPOINT" ]]; then
  echo "Unexpected endpoint: $ENDPOINT"
  exit 1
fi

if [[ "$STRIPE_KEY" != "$STRIPE_API_KEY" ]]; then
  echo "Unexpected Stripe key"
  exit 1
fi

if [[ -z "$ROTATION_ID" ]]; then
  echo "rotation_id was not generated"
  exit 1
fi

echo "Smoke test passed."

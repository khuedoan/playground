#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
export BAO_BIN="${BAO_BIN:-$ROOT_DIR/.tools/bin/bao}"
export SECRETSPEC_BIN="${SECRETSPEC_BIN:-$ROOT_DIR/.tools/bin/secretspec}"

if [[ ! -x "$BAO_BIN" ]]; then
  "$ROOT_DIR/scripts/install-openbao.sh"
fi
if [[ ! -x "$SECRETSPEC_BIN" ]]; then
  "$ROOT_DIR/scripts/install-secretspec.sh"
fi

if [[ -z "${BAO_ADDR:-}" ]]; then
  TEST_PORT="$((19000 + (RANDOM % 1000)))"
  export BAO_ADDR="http://127.0.0.1:${TEST_PORT}"
else
  TEST_PORT="${BAO_ADDR##*:}"
fi
export BAO_DEV_ROOT_TOKEN_ID="${BAO_DEV_ROOT_TOKEN_ID:-dev-only-root-token}"
export BAO_TOKEN="${BAO_TOKEN:-$BAO_DEV_ROOT_TOKEN_ID}"
export VAULT_TOKEN="$BAO_TOKEN"
export VAULT_ADDR="$BAO_ADDR"
export HTTP_PROXY=
export HTTPS_PROXY=
export ALL_PROXY=
export http_proxy=
export https_proxy=
export all_proxy=
export PAYMENTS_ENDPOINT="${PAYMENTS_ENDPOINT:-https://api.example.internal}"
export STRIPE_API_KEY="${STRIPE_API_KEY:-sk_test_example_key}"
export STRIPE_WEBHOOK_SECRET="${STRIPE_WEBHOOK_SECRET:-whsec_example_secret}"

"$ROOT_DIR/scripts/bootstrap.sh"

PROVIDER="${SECRETSPEC_PROVIDER:-openbao://127.0.0.1:${TEST_PORT}/apps?tls=false}"
PROFILE="${SECRETSPEC_PROFILE:-default}"

PAYMENTS_USER="$($SECRETSPEC_BIN get PAYMENTS_USERNAME --provider "$PROVIDER" --profile "$PROFILE")"
PAYMENTS_URL="$($SECRETSPEC_BIN get PAYMENTS_ENDPOINT --provider "$PROVIDER" --profile "$PROFILE")"
STRIPE_KEY="$($SECRETSPEC_BIN get STRIPE_API_KEY --provider "$PROVIDER" --profile "$PROFILE")"
ROTATION_ID="$($SECRETSPEC_BIN get STRIPE_ROTATION_ID --provider "$PROVIDER" --profile "$PROFILE")"

[[ "$PAYMENTS_USER" == "payments-service" ]] || { echo "Unexpected PAYMENTS_USERNAME"; exit 1; }
[[ "$PAYMENTS_URL" == "$PAYMENTS_ENDPOINT" ]] || { echo "Unexpected PAYMENTS_ENDPOINT"; exit 1; }
[[ "$STRIPE_KEY" == "$STRIPE_API_KEY" ]] || { echo "Unexpected STRIPE_API_KEY"; exit 1; }
[[ -n "$ROTATION_ID" ]] || { echo "Missing STRIPE_ROTATION_ID"; exit 1; }

echo "Smoke test passed (SecretSpec + OpenBao provider)."

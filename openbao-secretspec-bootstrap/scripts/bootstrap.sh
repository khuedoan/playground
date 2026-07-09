#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INSTALL_BAO_SCRIPT="$ROOT_DIR/scripts/install-openbao.sh"
INSTALL_SECRETSPEC_SCRIPT="$ROOT_DIR/scripts/install-secretspec.sh"

resolve_bin() {
  local name="$1"
  local local_path="$2"
  if command -v "$name" >/dev/null 2>&1; then
    command -v "$name"
    return 0
  fi
  if [[ -x "$local_path" ]]; then
    echo "$local_path"
    return 0
  fi
  return 1
}

ensure_bao_bin() {
  if BAO_BIN="$(resolve_bin bao "$ROOT_DIR/.tools/bin/bao")"; then
    export BAO_BIN
    return 0
  fi

  if [[ "${AUTO_INSTALL_BAO:-1}" == "1" ]]; then
    "$INSTALL_BAO_SCRIPT"
    BAO_BIN="$(resolve_bin bao "$ROOT_DIR/.tools/bin/bao")"
    export BAO_BIN
    return 0
  fi

  echo "bao not found"
  exit 1
}

ensure_secretspec_bin() {
  if SECRETSPEC_BIN="$(resolve_bin secretspec "$ROOT_DIR/.tools/bin/secretspec")"; then
    export SECRETSPEC_BIN
    return 0
  fi

  if [[ "${AUTO_INSTALL_SECRETSPEC:-1}" == "1" ]]; then
    "$INSTALL_SECRETSPEC_SCRIPT"
    SECRETSPEC_BIN="$(resolve_bin secretspec "$ROOT_DIR/.tools/bin/secretspec")"
    export SECRETSPEC_BIN
    return 0
  fi

  echo "secretspec not found"
  exit 1
}

start_dev_if_needed() {
  if "$BAO_BIN" status >/dev/null 2>&1; then
    return 0
  fi

  if [[ "${START_DEV:-1}" != "1" ]]; then
    echo "OpenBao not reachable at BAO_ADDR=$BAO_ADDR and START_DEV!=1"
    exit 1
  fi

  local listen_addr="$BAO_ADDR"
  listen_addr="${listen_addr#http://}"
  listen_addr="${listen_addr#https://}"

  nohup "$BAO_BIN" server -dev -dev-listen-address="$listen_addr" -dev-root-token-id="$BAO_DEV_ROOT_TOKEN_ID" \
    > "$ROOT_DIR/.openbao-dev.log" 2>&1 &
  sleep 2
  "$BAO_BIN" status >/dev/null
}

read_env_or_prompt() {
  local env_name="$1"
  local prompt="$2"
  local secret="$3"

  if [[ -n "${!env_name:-}" ]]; then
    printf '%s' "${!env_name}"
    return 0
  fi

  if [[ -t 0 ]]; then
    if [[ "$secret" == "1" ]]; then
      read -r -s -p "$prompt" value
      echo
    else
      read -r -p "$prompt" value
    fi
    printf '%s' "$value"
    return 0
  fi

  echo "Missing required env var: $env_name" >&2
  exit 1
}

generate_value() {
  local generator="$1"
  local length="$2"
  case "$generator" in
    password)
      python - "$length" <<'PY'
import secrets
import string
import sys
n = int(sys.argv[1])
alphabet = string.ascii_letters + string.digits + "_@#%+="
print("".join(secrets.choice(alphabet) for _ in range(n)), end="")
PY
      ;;
    uuid)
      python - <<'PY'
import uuid
print(uuid.uuid4(), end="")
PY
      ;;
    *) echo "Unknown generator: $generator" >&2; exit 1 ;;
  esac
}

provider_from_addr() {
  local addr="$1"
  local hostport="${addr#http://}"
  local tls="true"
  if [[ "$addr" == http://* ]]; then
    tls="false"
  elif [[ "$addr" == https://* ]]; then
    hostport="${addr#https://}"
  fi
  echo "openbao://${hostport}/${SECRETSPEC_MOUNT}?tls=${tls}"
}

seed_with_secretspec() {
  local file="$1"
  local count
  count="$(jq -r '.secrets | length' "$file")"

  pushd "$ROOT_DIR" >/dev/null
  for ((i = 0; i < count; i++)); do
    local key type value
    key="$(jq -r ".secrets[$i].key" "$file")"
    type="$(jq -r ".secrets[$i].type" "$file")"

    case "$type" in
      literal)
        value="$(jq -r ".secrets[$i].value" "$file")"
        ;;
      env)
        env_name="$(jq -r ".secrets[$i].name" "$file")"
        prompt="$(jq -r ".secrets[$i].prompt // \"Paste value for ${env_name}: \"" "$file")"
        secret="$(jq -r ".secrets[$i].secret // 1" "$file")"
        value="$(read_env_or_prompt "$env_name" "$prompt" "$secret")"
        ;;
      generate)
        generator="$(jq -r ".secrets[$i].generator // \"password\"" "$file")"
        length="$(jq -r ".secrets[$i].length // 32" "$file")"
        value="$(generate_value "$generator" "$length")"
        ;;
      *)
        echo "Unsupported type: $type"
        exit 1
        ;;
    esac

    "$SECRETSPEC_BIN" set "$key" "$value" \
      --provider "$SECRETSPEC_PROVIDER" \
      --profile "$SECRETSPEC_PROFILE" >/dev/null
    echo "Seeded via SecretSpec: $key"
  done

  "$SECRETSPEC_BIN" check --provider "$SECRETSPEC_PROVIDER" --profile "$SECRETSPEC_PROFILE" >/dev/null
  popd >/dev/null
}

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required"
  exit 1
fi

ensure_bao_bin
ensure_secretspec_bin

export BAO_ADDR="${BAO_ADDR:-http://127.0.0.1:8200}"
export BAO_DEV_ROOT_TOKEN_ID="${BAO_DEV_ROOT_TOKEN_ID:-dev-only-root-token}"
export BAO_TOKEN="${BAO_TOKEN:-$BAO_DEV_ROOT_TOKEN_ID}"
export VAULT_TOKEN="$BAO_TOKEN"
export VAULT_ADDR="$BAO_ADDR"
export NO_PROXY="${NO_PROXY:-127.0.0.1,localhost}"
export no_proxy="${no_proxy:-127.0.0.1,localhost}"
export HTTP_PROXY=
export HTTPS_PROXY=
export ALL_PROXY=
export http_proxy=
export https_proxy=
export all_proxy=

start_dev_if_needed

SECRETS_FILE="${SECRETS_FILE:-$ROOT_DIR/bootstrap-secrets.json}"
SECRETSPEC_PROFILE="${SECRETSPEC_PROFILE:-$(jq -r '.profile // "default"' "$SECRETS_FILE")}"
SECRETSPEC_MOUNT="${SECRETSPEC_MOUNT:-apps}"
SECRETSPEC_PROVIDER="${SECRETSPEC_PROVIDER:-$(provider_from_addr "$BAO_ADDR")}" 

"$BAO_BIN" secrets enable -path="$SECRETSPEC_MOUNT" kv-v2 >/dev/null 2>&1 || true

seed_with_secretspec "$SECRETS_FILE"

echo "Bootstrap complete via secretspec.dev"
echo "SECRETSPEC_PROVIDER=$SECRETSPEC_PROVIDER"

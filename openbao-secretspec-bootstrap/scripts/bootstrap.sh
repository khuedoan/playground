#!/usr/bin/env bash
set -euo pipefail

# One-command OpenBao bootstrap.
# - Resolves/install OpenBao CLI
# - Optionally starts a local dev server
# - Enables kv-v2 mount
# - Loads secrets from bootstrap-secrets.json
#
# Usage:
#   ./scripts/bootstrap.sh
#
# Optional env:
#   BAO_BIN=./.tools/bin/bao
#   BAO_ADDR=http://127.0.0.1:8200
#   BAO_DEV_ROOT_TOKEN_ID=dev-only-root-token
#   BAO_TOKEN=...
#   START_DEV=1
#   AUTO_INSTALL_BAO=1
#   SECRETS_FILE=./bootstrap-secrets.json

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INSTALL_SCRIPT="$ROOT_DIR/scripts/install-openbao.sh"

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

  if [[ -x "$ROOT_DIR/.tools/bin/bao" ]]; then
    echo "$ROOT_DIR/.tools/bin/bao"
    return 0
  fi

  return 1
}

ensure_bao_bin() {
  if BAO_BIN="$(resolve_bao_bin)"; then
    export BAO_BIN
    return 0
  fi

  if [[ "${AUTO_INSTALL_BAO:-1}" == "1" ]]; then
    echo "OpenBao CLI not found; installing locally..."
    "$INSTALL_SCRIPT"
    BAO_BIN="$(resolve_bao_bin)"
    export BAO_BIN
    return 0
  fi

  echo "OpenBao CLI not found. Install it or run ./scripts/install-openbao.sh"
  exit 1
}

ensure_deps() {
  if ! command -v jq >/dev/null 2>&1; then
    echo "jq not found in PATH. Install jq before continuing."
    exit 1
  fi
}

start_dev_if_needed() {
  local start_dev="${START_DEV:-1}"

  if "$BAO_BIN" status >/dev/null 2>&1; then
    return 0
  fi

  if [[ "$start_dev" != "1" ]]; then
    echo "OpenBao not reachable at BAO_ADDR=$BAO_ADDR and START_DEV!=1"
    exit 1
  fi

  if pgrep -f "${BAO_BIN} server -dev" >/dev/null 2>&1; then
    sleep 1
  else
    echo "Starting local OpenBao dev server with ${BAO_BIN}..."
    nohup "$BAO_BIN" server -dev -dev-root-token-id="$BAO_DEV_ROOT_TOKEN_ID" \
      > "$ROOT_DIR/.openbao-dev.log" 2>&1 &
    sleep 2
  fi

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

length = int(sys.argv[1])
alphabet = string.ascii_letters + string.digits + "_@#%+="
print("".join(secrets.choice(alphabet) for _ in range(length)), end="")
PY
      ;;
    uuid)
      if [[ -r /proc/sys/kernel/random/uuid ]]; then
        cat /proc/sys/kernel/random/uuid
      else
        python - <<'PY'
import uuid
print(uuid.uuid4())
PY
      fi
      ;;
    *)
      echo "Unknown generator: $generator" >&2
      exit 1
      ;;
  esac
}

write_secrets_from_file() {
  local file="$1"

  if [[ ! -f "$file" ]]; then
    echo "Secrets file not found: $file"
    exit 1
  fi

  local mount
  mount="$(jq -r '.mount // "apps"' "$file")"

  "$BAO_BIN" secrets enable -path="$mount" kv-v2 >/dev/null 2>&1 || true

  local count
  count="$(jq -r '.secrets | length' "$file")"

  for ((i = 0; i < count; i++)); do
    local rel_path
    rel_path="$(jq -r ".secrets[$i].path" "$file")"

    local -a kv_args=()
    mapfile -t keys < <(jq -r ".secrets[$i].fields | keys[]" "$file")

    for key in "${keys[@]}"; do
      local type
      type="$(jq -r --arg key "$key" ".secrets[$i].fields[\$key].type" "$file")"

      local value
      case "$type" in
        literal)
          value="$(jq -r --arg key "$key" ".secrets[$i].fields[\$key].value" "$file")"
          ;;
        env)
          local env_name prompt secret
          env_name="$(jq -r --arg key "$key" ".secrets[$i].fields[\$key].name" "$file")"
          prompt="$(jq -r --arg key "$key" ".secrets[$i].fields[\$key].prompt // \"Paste value for ${env_name}: \"" "$file")"
          secret="$(jq -r --arg key "$key" ".secrets[$i].fields[\$key].secret // 1" "$file")"
          value="$(read_env_or_prompt "$env_name" "$prompt" "$secret")"
          ;;
        generate)
          local generator length
          generator="$(jq -r --arg key "$key" ".secrets[$i].fields[\$key].generator // \"password\"" "$file")"
          length="$(jq -r --arg key "$key" ".secrets[$i].fields[\$key].length // 32" "$file")"
          value="$(generate_value "$generator" "$length")"
          ;;
        *)
          echo "Unsupported field type '$type' for $rel_path:$key"
          exit 1
          ;;
      esac

      kv_args+=("$key=$value")
    done

    "$BAO_BIN" kv put "$mount/$rel_path" "${kv_args[@]}" >/dev/null
    echo "Seeded: $mount/data/$rel_path"
  done
}

ensure_bao_bin
ensure_deps

export BAO_ADDR="${BAO_ADDR:-http://127.0.0.1:8200}"
export BAO_DEV_ROOT_TOKEN_ID="${BAO_DEV_ROOT_TOKEN_ID:-dev-only-root-token}"
export BAO_TOKEN="${BAO_TOKEN:-$BAO_DEV_ROOT_TOKEN_ID}"

start_dev_if_needed

SECRETS_FILE="${SECRETS_FILE:-$ROOT_DIR/bootstrap-secrets.json}"
write_secrets_from_file "$SECRETS_FILE"

echo
printf 'Bootstrap complete.\n'
printf 'BAO_BIN=%s\n' "$BAO_BIN"
printf 'BAO_ADDR=%s\n' "$BAO_ADDR"
printf 'BAO_TOKEN=%s\n' "$BAO_TOKEN"
printf 'SECRETS_FILE=%s\n' "$SECRETS_FILE"

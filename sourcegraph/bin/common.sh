#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if [[ -f "${ROOT_DIR}/.env" ]]; then
  set -a
  # shellcheck disable=SC1091
  source "${ROOT_DIR}/.env"
  set +a
fi

compose() {
  docker compose --env-file "${ROOT_DIR}/.env" \
    -f "${ROOT_DIR}/compose.yaml" \
    -f "${ROOT_DIR}/compose.override.yaml" \
    "$@"
}

#!/usr/bin/env bash

set -euo pipefail

# shellcheck disable=SC1091
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"

echo "Sourcegraph URL: ${SOURCEGRAPH_EXTERNAL_URL:-http://127.0.0.1:7080}"
echo "MCP URL:        http://${MCP_BIND_ADDRESS:-127.0.0.1}:${MCP_PORT:-7081}/mcp"
echo

compose ps
echo

curl -fsS -I "${SOURCEGRAPH_EXTERNAL_URL:-http://127.0.0.1:7080}/" | sed -n '1,5p'
echo
curl -fsS "http://${MCP_BIND_ADDRESS:-127.0.0.1}:${MCP_PORT:-7081}/healthz"
echo

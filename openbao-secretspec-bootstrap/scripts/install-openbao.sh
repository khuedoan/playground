#!/usr/bin/env bash
set -euo pipefail

# Installs a local OpenBao CLI binary into ./.tools/bin (repo-local).
# Defaults to v2.5.3 for reproducibility; override with BAO_VERSION.

BAO_VERSION="${BAO_VERSION:-2.5.3}"
OS="${OS:-Linux}"
ARCH_RAW="$(uname -m)"

case "$ARCH_RAW" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="arm64" ;;
  *)
    echo "Unsupported architecture: $ARCH_RAW"
    exit 1
    ;;
esac

TOOLS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)/.tools/bin"
mkdir -p "$TOOLS_DIR"

TARBALL="bao_${BAO_VERSION}_${OS}_${ARCH}.tar.gz"
URL="https://github.com/openbao/openbao/releases/download/v${BAO_VERSION}/${TARBALL}"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Downloading ${URL}"
curl -fsSL "$URL" -o "$TMP_DIR/$TARBALL"

tar -xzf "$TMP_DIR/$TARBALL" -C "$TMP_DIR"

if [[ ! -f "$TMP_DIR/bao" ]]; then
  echo "Expected 'bao' binary not found in archive"
  exit 1
fi

install -m 0755 "$TMP_DIR/bao" "$TOOLS_DIR/bao"

echo "Installed: $TOOLS_DIR/bao"
echo "Run with: BAO_BIN=$TOOLS_DIR/bao"

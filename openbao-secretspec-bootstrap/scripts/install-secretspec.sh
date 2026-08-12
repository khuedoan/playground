#!/usr/bin/env bash
set -euo pipefail

VERSION="${SECRETSPEC_VERSION:-0.8.2}"
ARCH_RAW="$(uname -m)"
OS_RAW="$(uname -s)"

case "$OS_RAW" in
  Linux) OS="unknown-linux-gnu" ;;
  Darwin) OS="apple-darwin" ;;
  *) echo "Unsupported OS: $OS_RAW"; exit 1 ;;
esac

case "$ARCH_RAW" in
  x86_64|amd64) ARCH="x86_64" ;;
  aarch64|arm64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH_RAW"; exit 1 ;;
esac

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TOOLS_DIR="$ROOT_DIR/.tools/bin"
mkdir -p "$TOOLS_DIR"

ASSET="secretspec-${ARCH}-${OS}.tar.xz"
URL="https://github.com/cachix/secretspec/releases/download/v${VERSION}/${ASSET}"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' EXIT

echo "Downloading ${URL}"
curl -fsSL "$URL" -o "$TMP_DIR/$ASSET"

tar -xJf "$TMP_DIR/$ASSET" -C "$TMP_DIR"
BIN_PATH="$(find "$TMP_DIR" -type f -name secretspec | head -n 1)"
if [[ -z "${BIN_PATH:-}" ]]; then
  echo "Expected secretspec binary not found"
  exit 1
fi

install -m 0755 "$BIN_PATH" "$TOOLS_DIR/secretspec"

echo "Installed: $TOOLS_DIR/secretspec"

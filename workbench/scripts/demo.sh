#!/usr/bin/env bash
set -Eeuo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$project_root"

if [ ! -c /dev/kvm ]; then
  echo "A KVM-capable Linux host is required for the real demo; no simulated recording is used." >&2
  exit 1
fi

if ! systemctl is-active --quiet workbench-host-agent.service; then
  echo "workbench-host-agent.service must be running." >&2
  exit 1
fi

control_url="${CONTROL_URL:-http://127.0.0.1:4000}"
if ! curl --fail --silent "$control_url" >/dev/null; then
  echo "The Phoenix control plane is not reachable at $control_url." >&2
  exit 1
fi

npm --prefix demo ci
CONTROL_URL="$control_url" ARTIFACTS_DIR="$project_root/demo/artifacts" npm --prefix demo run record

if command -v ffmpeg >/dev/null 2>&1; then
  ffmpeg -y -i demo/artifacts/workbench-demo.webm \
    -c:v libx264 -pix_fmt yuv420p -movflags +faststart \
    demo/artifacts/workbench-demo.mp4
  echo "Demo video: $project_root/demo/artifacts/workbench-demo.mp4"
else
  echo "Demo video: $project_root/demo/artifacts/workbench-demo.webm"
fi

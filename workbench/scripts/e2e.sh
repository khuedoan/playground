#!/usr/bin/env bash
set -Eeuo pipefail

project_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
artifacts="${ARTIFACTS_DIR:-$project_root/demo/artifacts}"
control_url="${CONTROL_URL:-http://127.0.0.1:4000}"
host_url="${HOST_AGENT_URL:-http://127.0.0.1:9090}"

mkdir -p "$artifacts"

if ((${#SECRET_KEY_BASE} < 64)); then
  echo "SECRET_KEY_BASE must contain at least 64 bytes" >&2
  exit 1
fi

if [ ! -c /dev/kvm ]; then
  echo "the end-to-end suite requires /dev/kvm" >&2
  exit 1
fi
if ! systemctl is-active --quiet workbench-host-agent.service; then
  echo "workbench-host-agent.service is not active" >&2
  exit 1
fi

cd "$project_root/apps/control_plane"
MIX_ENV=prod mix ecto.setup
MIX_ENV=prod mix phx.server > "$artifacts/phoenix.log" 2>&1 &
phoenix_pid=$!
trap 'kill "$phoenix_pid" 2>/dev/null || true' EXIT

for attempt in $(seq 1 120); do
  if curl --fail --silent "$control_url" >/dev/null; then
    break
  fi
  if [ "$attempt" -eq 120 ]; then
    echo "Phoenix did not become ready" >&2
    exit 1
  fi
  sleep 1
done

cd "$project_root"
ARTIFACTS_DIR="$artifacts" CONTROL_URL="$control_url" ./scripts/demo.sh

curl --fail --silent "$host_url/v1/workspaces" > "$artifacts/workspaces.json"
workspace_id="$(jq -er 'if length == 1 then .[0].workspace_id else error("expected one workspace") end' "$artifacts/workspaces.json")"
agent_url="$(jq -er '.[0].agent_url' "$artifacts/workspaces.json")"
code_url="$(jq -er '.[0].code_url' "$artifacts/workspaces.json")"
desktop_url="$(jq -er '.[0].desktop_url' "$artifacts/workspaces.json")"
vm_name="workbench-$workspace_id"

for url in "$agent_url/healthz" "$code_url" "$desktop_url"; do
  for attempt in $(seq 1 120); do
    if curl --fail --silent --location "$url" >/dev/null; then
      break
    fi
    if [ "$attempt" -eq 120 ]; then
      echo "endpoint did not become ready: $url" >&2
      exit 1
    fi
    sleep 1
  done
done

verification_command='test -s /workspace/workbench-demo.txt && systemctl is-active workbench-mock-llm.service workbench-sway.service workbench-wayvnc.service workbench-novnc.service code-server.service && pgrep --full --list-full "/bin/blender( |$)"'
jq -nc --arg command "$verification_command" '{command: $command, cwd: "/workspace", timeout_seconds: 60}' |
  curl --fail --silent -H 'content-type: application/json' --data-binary @- "$agent_url/v1/exec" |
  tee "$artifacts/guest-verification.json" |
  jq -e '.exit_code == 0 and .timed_out == false' >/dev/null

sudo systemctl restart "microvm@$vm_name.service"
for attempt in $(seq 1 180); do
  if curl --fail --silent "$agent_url/healthz" >/dev/null; then
    break
  fi
  if [ "$attempt" -eq 180 ]; then
    echo "guest agent did not return after restart" >&2
    exit 1
  fi
  sleep 1
done

jq -nc '{command: "test -s /workspace/workbench-demo.txt", cwd: "/workspace", timeout_seconds: 30}' |
  curl --fail --silent -H 'content-type: application/json' --data-binary @- "$agent_url/v1/exec" |
  tee "$artifacts/persistence-verification.json" |
  jq -e '.exit_code == 0 and .timed_out == false' >/dev/null

printf 'Real MicroVM boot, mock-API Pi tool use, GUI services, code-server, and restart persistence passed.\n' |
  tee "$artifacts/e2e-result.txt"

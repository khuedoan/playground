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
jq -e '
  length == 2 and
  all(.[]; .actual_state == "running") and
  .[0].slot_id != .[1].slot_id and
  .[0].ip_address != .[1].ip_address
' "$artifacts/workspaces.json" >/dev/null

postgres_url="${POSTGRES_URL:-postgresql://postgres:postgres@127.0.0.1/workbench_e2e}"
workspace_id="$(psql "$postgres_url" -Atc "select id from workspaces where title like 'MicroVM demo %' order by inserted_at desc limit 1")"
parallel_id="$(psql "$postgres_url" -Atc "select id from workspaces where title like 'Parallel review %' order by inserted_at desc limit 1")"
test -n "$workspace_id"
test -n "$parallel_id"

agent_url="$(jq -er --arg id "$workspace_id" '.[] | select(.workspace_id == $id) | .agent_url' "$artifacts/workspaces.json")"
code_url="$(jq -er --arg id "$workspace_id" '.[] | select(.workspace_id == $id) | .code_url' "$artifacts/workspaces.json")"
desktop_url="$(jq -er --arg id "$workspace_id" '.[] | select(.workspace_id == $id) | .desktop_url' "$artifacts/workspaces.json")"
parallel_agent_url="$(jq -er --arg id "$parallel_id" '.[] | select(.workspace_id == $id) | .agent_url' "$artifacts/workspaces.json")"
vm_name="$(jq -er --arg id "$workspace_id" '.[] | select(.workspace_id == $id) | .slot_id' "$artifacts/workspaces.json")"

for url in "$agent_url/healthz" "$parallel_agent_url/healthz" "$code_url" "$desktop_url"; do
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

parallel_verification_command='test ! -e /workspace/workbench-demo.txt && systemctl is-active workbench-mock-llm.service workbench-sway.service workbench-wayvnc.service workbench-novnc.service code-server.service'
jq -nc --arg command "$parallel_verification_command" '{command: $command, cwd: "/workspace", timeout_seconds: 60}' |
  curl --fail --silent -H 'content-type: application/json' --data-binary @- "$parallel_agent_url/v1/exec" |
  tee "$artifacts/parallel-isolation-verification.json" |
  jq -e '.exit_code == 0 and .timed_out == false' >/dev/null

read -r workspace_count max_boot_ms <<< "$(psql "$postgres_url" -At -F ' ' -c "select count(*), coalesce(max(boot_ms), 0) from workspaces where status = 'running';")"
test "$workspace_count" -eq 2
if [ "$max_boot_ms" -ge 1000 ]; then
  echo "warm-pool lease took ${max_boot_ms}ms; expected less than 1000ms" >&2
  exit 1
fi

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

printf 'Two concurrent warm-pool MicroVM threads, sub-second leases, isolation, mock-API tool use, Wayland GUI, code-server, and restart persistence passed.\n' |
  tee "$artifacts/e2e-result.txt"

#!/usr/bin/env bash
set -Eeuo pipefail

if [ "$(id -u)" -ne 0 ]; then
  echo "collect-diagnostics.sh must run as root" >&2
  exit 1
fi

artifacts="${1:?pass the artifact directory}"
install -d -m 0755 "$artifacts"

journalctl --no-pager -u workbench-host-agent.service > "$artifacts/host-agent.log" || true
journalctl --no-pager -u 'microvm@*.service' > "$artifacts/microvm.log" || true
journalctl --no-pager -u 'microvm-virtiofsd@*.service' > "$artifacts/virtiofsd.log" || true
journalctl --no-pager -u 'microvm-tap-interfaces@*.service' > "$artifacts/tap.log" || true
systemctl --no-pager --full status workbench-host-agent.service 'microvm@*.service' > "$artifacts/systemd-status.txt" || true
ip address show > "$artifacts/ip-address.txt" || true
ip route show > "$artifacts/ip-route.txt" || true
curl --silent http://127.0.0.1:9090/v1/workspaces > "$artifacts/host-state.json" || true

agent_url="$(jq -r '.[0].agent_url // empty' "$artifacts/host-state.json" 2>/dev/null || true)"
if [ -n "$agent_url" ]; then
  curl --silent --show-error "$agent_url/healthz" > "$artifacts/guest-health.json" || true
  diagnostic_command='curl --fail --silent http://127.0.0.1:8080/health; echo; test -r /home/workbench/.pi/agent/models.json && echo models_config=readable || echo models_config=unreadable; test -w /workspace/.pi/sessions && echo session_dir=writable || echo session_dir=unwritable; test -s /workspace/workbench-demo.txt && echo demo_file=present || echo demo_file=missing; systemctl --no-pager --full status workbench-mock-llm.service workbench-sway.service workbench-wayvnc.service workbench-novnc.service code-server.service || true; ss -ltn || true'
  jq -nc --arg command "$diagnostic_command" \
    '{command: $command, cwd: "/workspace", timeout_seconds: 30}' |
    curl --silent --show-error -H 'content-type: application/json' --data-binary @- \
      "$agent_url/v1/exec" > "$artifacts/guest-environment.json" || true
fi

chmod -R a+rX "$artifacts"

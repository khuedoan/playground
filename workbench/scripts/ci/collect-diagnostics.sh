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

host_agent_pid="$(systemctl show workbench-host-agent.service --property MainPID --value 2>/dev/null || true)"
{
  if [ -n "$host_agent_pid" ] && [ "$host_agent_pid" != "0" ] && [ -r "/proc/$host_agent_pid/environ" ]; then
    if tr '\0' '\n' < "/proc/$host_agent_pid/environ" | grep -q '^GITHUB_MODELS_TOKEN=.'; then
      echo 'host_model_credentials=set'
    else
      echo 'host_model_credentials=unset'
    fi
  fi
  find /run/workbench/credentials -maxdepth 2 -type f -name model.env \
    -printf 'credential_file=%p mode=%m uid=%U gid=%G bytes=%s\n' 2>/dev/null || true
  find /run/workbench/credentials -maxdepth 2 -type f -name model.env -exec \
    sh -c 'sed -n "s/^\([A-Z0-9_]*\)=.*/credential_key=\1/p" "$1"' _ {} \; 2>/dev/null || true
} > "$artifacts/credential-status.txt"

agent_url="$(jq -r '.[0].agent_url // empty' "$artifacts/host-state.json" 2>/dev/null || true)"
if [ -n "$agent_url" ]; then
  curl --silent --show-error "$agent_url/healthz" > "$artifacts/guest-health.json" || true
  diagnostic_command='printf "github_models_token=%s\n" "${GITHUB_MODELS_TOKEN:+set}"; test -r /home/workbench/.pi/agent/models.json && echo models_config=readable || echo models_config=unreadable; test -w /workspace/.pi/sessions && echo session_dir=writable || echo session_dir=unwritable'
  jq -nc --arg command "$diagnostic_command" \
    '{command: $command, cwd: "/workspace", timeout_seconds: 30}' |
    curl --silent --show-error -H 'content-type: application/json' --data-binary @- \
      "$agent_url/v1/exec" > "$artifacts/guest-environment.json" || true
fi

chmod -R a+rX "$artifacts"

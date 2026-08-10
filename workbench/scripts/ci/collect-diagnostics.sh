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

chmod -R a+rX "$artifacts"

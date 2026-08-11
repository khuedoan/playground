#!/usr/bin/env bash
set -Eeuo pipefail

if [ "$(id -u)" -ne 0 ]; then
  echo "prepare-host.sh must run as root" >&2
  exit 1
fi

: "${WORKBENCH_HOST_AGENT_BIN:?set WORKBENCH_HOST_AGENT_BIN}"
: "${WORKBENCH_MICROVM_BIN:?set WORKBENCH_MICROVM_BIN}"
: "${WORKBENCH_FLAKE_ROOT:?set WORKBENCH_FLAKE_ROOT}"
mock_llm="${WORKBENCH_E2E_MOCK_LLM:-false}"
warm_pool_size="${WORKBENCH_WARM_POOL_SIZE:-2}"
pool_vcpus="${WORKBENCH_POOL_VCPUS:-${WORKBENCH_VCPUS:-4}}"
pool_memory_mib="${WORKBENCH_POOL_MEMORY_MIB:-${WORKBENCH_MEMORY_MIB:-8192}}"
pool_disk_gib="${WORKBENCH_POOL_DISK_GIB:-${WORKBENCH_DISK_GIB:-40}}"
case "$mock_llm" in
  true|false) ;;
  *) echo "WORKBENCH_E2E_MOCK_LLM must be true or false" >&2; exit 1 ;;
esac
for value in "$warm_pool_size" "$pool_vcpus" "$pool_memory_mib" "$pool_disk_gib"; do
  case "$value" in
    ''|*[!0-9]*) echo "warm-pool capacity and resources must be integers" >&2; exit 1 ;;
  esac
done

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
unit_dir="$script_dir/systemd"

if [ ! -c /dev/kvm ]; then
  echo "GitHub runner does not expose /dev/kvm" >&2
  exit 1
fi

getent group kvm >/dev/null || groupadd --system kvm
if id microvm >/dev/null 2>&1; then
  usermod --append --groups kvm microvm
else
  useradd --system --gid kvm --home-dir /var/lib/microvms --no-create-home --shell /usr/sbin/nologin microvm
fi

chown root:kvm /dev/kvm
chmod 0660 /dev/kvm
if [ ! -c /dev/net/tun ]; then
  modprobe tun
fi
test -c /dev/net/tun

install -d -m 0750 -o microvm -g kvm /var/lib/microvms
install -d -m 0700 -o root -g root /var/lib/workbench /var/lib/workbench/specs
install -d -m 0700 -o root -g root /run/workbench-e2e

if ! ip link show workbench0 >/dev/null 2>&1; then
  ip link add workbench0 type bridge
fi
ip address replace 10.88.0.1/16 dev workbench0
ip link set workbench0 up
sysctl -w net.ipv4.ip_forward=1 >/dev/null

outbound_interface="$(ip route show default | awk '/default/ { print $5; exit }')"
if [ -z "$outbound_interface" ]; then
  echo "could not determine the runner's outbound interface" >&2
  exit 1
fi

iptables -t nat -C POSTROUTING -s 10.88.0.0/16 -o "$outbound_interface" -j MASQUERADE 2>/dev/null || \
  iptables -t nat -A POSTROUTING -s 10.88.0.0/16 -o "$outbound_interface" -j MASQUERADE
iptables -C FORWARD -i workbench0 -j ACCEPT 2>/dev/null || \
  iptables -A FORWARD -i workbench0 -j ACCEPT
iptables -C FORWARD -o workbench0 -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT 2>/dev/null || \
  iptables -A FORWARD -o workbench0 -m conntrack --ctstate RELATED,ESTABLISHED -j ACCEPT

install -Dm0755 "$script_dir/attach-taps.sh" /usr/local/libexec/workbench-attach-taps
for unit in "$unit_dir"/*.service; do
  install -Dm0644 "$unit" "/etc/systemd/system/${unit##*/}"
done

umask 077
{
  printf 'WORKBENCH_HOST_AGENT_BIN=%s\n' "$WORKBENCH_HOST_AGENT_BIN"
  printf 'WORKBENCH_HOST_LISTEN=127.0.0.1:9090\n'
  printf 'WORKBENCH_HOST_STATE=/var/lib/workbench/state.json\n'
  printf 'WORKBENCH_MICROVM=%s\n' "$WORKBENCH_MICROVM_BIN"
  printf 'WORKBENCH_SYSTEMCTL=/usr/bin/systemctl\n'
  printf 'WORKBENCH_FLAKE_ROOT=%s\n' "$WORKBENCH_FLAKE_ROOT"
  printf 'WORKBENCH_SPEC_ROOT=/var/lib/workbench/specs\n'
  printf 'WORKBENCH_MICROVM_STATE_ROOT=/var/lib/microvms\n'
  printf 'WORKBENCH_POOL_STATE=/var/lib/workbench/pool.json\n'
  printf 'WORKBENCH_WARM_POOL_SIZE=%s\n' "$warm_pool_size"
  printf 'WORKBENCH_POOL_VCPUS=%s\n' "$pool_vcpus"
  printf 'WORKBENCH_POOL_MEMORY_MIB=%s\n' "$pool_memory_mib"
  printf 'WORKBENCH_POOL_DISK_GIB=%s\n' "$pool_disk_gib"
  printf 'WORKBENCH_GUEST_HEALTH_TIMEOUT_SECONDS=300\n'
  printf 'WORKBENCH_E2E_MOCK_LLM=%s\n' "$mock_llm"
  printf 'RUST_LOG=info\n'
} > /run/workbench-e2e/host.env

systemctl daemon-reload
systemctl restart workbench-dns.service
systemctl restart workbench-host-agent.service

for attempt in $(seq 1 1800); do
  if curl --fail --silent http://127.0.0.1:9090/healthz >/dev/null; then
    exit 0
  fi
  sleep 1
done

systemctl status --no-pager workbench-host-agent.service >&2
exit 1

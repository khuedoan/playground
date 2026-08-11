#!/usr/bin/env bash
set -Eeuo pipefail

for path in /sys/class/net/wb-*; do
  if [ -e "$path" ]; then
    interface="${path##*/}"
    ip link set "$interface" master workbench0
  fi
done

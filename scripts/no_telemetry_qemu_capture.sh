#!/bin/sh
# NONOS Operating System
# Copyright (C) 2026 NONOS Contributors
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.
#
# Boot an image with its network captured, then judge every frame the guest
# sent with tools/nonos-pcap-egress: no DNS, NTP or discovery packet, no
# IPv6, no burned-in MAC, no DHCP option that names the machine, and no TLS
# server name or HTTP host but the anonymity networks' pinned bootstrap.
#
# A networked boot of this design is not silent: DHCP, and the Nym and Anyone
# clients reaching their directories, go out on every boot. The earlier form
# of this check passed only on an empty capture, so it could pass only on a
# boot whose network never came up.
#
#   NONOS_NO_TELEMETRY_IMAGE    a sealed nonos.img to boot headless through
#                               tools/nonos-qemu (development images included)
#   NONOS_NO_TELEMETRY_BOOT_CMD a boot command of your own, which must write
#                               the capture to $NONOS_NO_TELEMETRY_PCAP
#   NONOS_NO_TELEMETRY_TIMEOUT  seconds the boot runs (default 180)
#
# Exit 0 pass, 1 a leak (the report names it), 2 no capture to judge.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"

out=${NONOS_NO_TELEMETRY_OUT:-target/no-telemetry}
timeout_secs=${NONOS_NO_TELEMETRY_TIMEOUT:-180}
pcap=${NONOS_NO_TELEMETRY_PCAP:-$out/qemu-net.pcap}
report=$out/report.json
image=${NONOS_NO_TELEMETRY_IMAGE:-}
boot_cmd=${NONOS_NO_TELEMETRY_BOOT_CMD:-}

mkdir -p "$out" "$(dirname "$pcap")"
rm -f "$pcap" "$report"

if [ -z "$boot_cmd" ] && [ -n "$image" ]; then
  boot_cmd="python3 tools/nonos-qemu --image '$image' --fresh --headless --serial '$out/serial.log' --timeout $timeout_secs --capture '$pcap'"
fi
if [ -z "$boot_cmd" ]; then
  boot_cmd="make QEMU_NET_CAPTURE=$pcap nonos-mk-run-serial-net"
fi

set +e
if command -v timeout >/dev/null 2>&1; then
  timeout "$((timeout_secs + 60))" sh -c "$boot_cmd"
elif command -v gtimeout >/dev/null 2>&1; then
  gtimeout "$((timeout_secs + 60))" sh -c "$boot_cmd"
else
  perl -e 'alarm shift; exec @ARGV or die "exec failed: $!"' "$((timeout_secs + 60))" sh -c "$boot_cmd"
fi
rc=$?
set -e

if [ ! -s "$pcap" ]; then
  printf '{"status":"gap","reason":"no capture","exit":%s,"pcap":"%s"}\n' "$rc" "$pcap" > "$report"
  printf '%s\n' "no-telemetry capture: GAP no capture written (boot exit $rc)"
  exit 2
fi

# The names the anonymity networks may be reached by in the clear: the
# bootstrap hosts their capsules pin by address and check by certificate.
names=$(sed -n 's/.*("\([a-z0-9.-]*\)", \[[0-9, ]*\]).*/\1/p' userland/capsule_net_nym/src/directory_sync/pinned.rs)
# The addresses they are pinned at: Nym's directory API and the bootstrap
# gateways it enters the mixnet through (it reaches no mix node itself), and
# Anyone's directory authorities. The relays Anyone then uses are learned
# from the consensus those authorities send, inside the capture itself.
ips=$(cat userland/capsule_net_nym/src/directory_sync/live.rs userland/capsule_net_nym/src/state/bootstrap.rs \
  userland/capsule_net_anon/src/directory/authority/list.rs \
  | sed -n 's/.*\[\([0-9]\{1,3\}\), \([0-9]\{1,3\}\), \([0-9]\{1,3\}\), \([0-9]\{1,3\}\)\].*/\1.\2.\3.\4/p' | sort -u)
set --
for n in $names; do set -- "$@" --allow-name "$n"; done
for ip in $ips; do set -- "$@" --allow "$ip"; done

set +e
python3 tools/nonos-pcap-egress "$pcap" --json "$report" "$@"
verdict=$?
set -e
case $verdict in
  0) printf '%s\n' "no-telemetry capture: PASS (report $report)" ;;
  1) printf '%s\n' "no-telemetry capture: FAIL (report $report)" ;;
  *) printf '%s\n' "no-telemetry capture: GAP the capture could not be read"; exit 2 ;;
esac
exit "$verdict"

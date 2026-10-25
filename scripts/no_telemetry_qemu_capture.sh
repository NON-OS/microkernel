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
# sent with tools/nonos-pcap-egress (tools/nonos_egress/judge.py lists the
# rules): no DNS, NTP or discovery packet, no IPv6, no ethertype but IPv4 and
# ARP, no burned-in MAC, no DHCP option or address that names the machine,
# and no peer, TLS name or HTTP host but the pinned bootstrap the committed
# allow-list (tools/nonos_egress/allowlist.json) names, and the relays its
# authorities' consensus names.
#
# A networked boot of this design is not silent: DHCP, and the Nym and Anyone
# clients reaching their directories, go out on every boot. A capture with no
# guest frame is EMPTY, and one with no DHCP or no pinned peer reached is
# INCOMPLETE: neither passes. So does a boot that stopped before its time.
#
#   NONOS_NO_TELEMETRY_IMAGE    a sealed nonos.img to boot headless through
#                               tools/nonos-qemu (development images included)
#   NONOS_NO_TELEMETRY_BOOT_CMD a boot command of your own, which must write
#                               the egress to $NONOS_NO_TELEMETRY_PCAP and the
#                               ingress beside it as -in.pcap, and exit 0 only
#                               when the boot ran its course
#   NONOS_NO_TELEMETRY_TIMEOUT  seconds the boot runs (default 180)
#   NONOS_NO_TELEMETRY_EXPECT   a serial line to wait for instead of the timeout
#   NONOS_QEMU_LANES            tools/nonos-qemu lanes, space separated. A
#                               release image refuses device DMA without an
#                               IOMMU, so under QEMU give it `iommu=intel`.
#
# Exit 0 pass, 1 a leak (the report names it), 2 no capture or allow-list to
# judge by, 3 empty, 4 incomplete, 5 the boot stopped before its time.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"

out=${NONOS_NO_TELEMETRY_OUT:-target/no-telemetry}
timeout_secs=${NONOS_NO_TELEMETRY_TIMEOUT:-180}
pcap=${NONOS_NO_TELEMETRY_PCAP:-$out/qemu-net.pcap}
report=$out/report.json
image=${NONOS_NO_TELEMETRY_IMAGE:-}
boot_cmd=${NONOS_NO_TELEMETRY_BOOT_CMD:-}

ingress=${pcap%.pcap}-in.pcap
expect=${NONOS_NO_TELEMETRY_EXPECT:-}

mkdir -p "$out" "$(dirname "$pcap")"

# The pinned peers come from the committed allow-list, which a reviewer read;
# the sources saying otherwise stops the run until the list is committed anew.
if ! python3 tools/nonos-pcap-egress --check-allowlist; then
  printf '{"status":"gap","reason":"allow-list differs from the sources"}\n' > "$report"
  exit 2
fi
rm -f "$pcap" "$ingress" "$report"

lanes=""
for l in ${NONOS_QEMU_LANES:-}; do lanes="$lanes --lane $l"; done
if [ -n "$expect" ]; then lanes="$lanes --expect '$expect'"; fi
if [ -z "$boot_cmd" ] && [ -n "$image" ]; then
  boot_cmd="python3 tools/nonos-qemu --image '$image' --fresh --headless --serial '$out/serial.log' --timeout $timeout_secs --capture '$pcap'$lanes"
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

if [ ! -s "$pcap" ] || [ ! -s "$ingress" ]; then
  printf '{"status":"gap","reason":"no capture","exit":%s,"pcap":"%s"}\n' "$rc" "$pcap" > "$report"
  printf '%s\n' "no-telemetry capture: GAP no capture written (boot exit $rc)"
  exit 2
fi

set +e
python3 tools/nonos-pcap-egress "$pcap" --ingress "$ingress" --json "$report"
verdict=$?
set -e
# tools/nonos-qemu exits 0 only for a boot still running at the timeout, or
# one that printed --expect; anything else stopped before its time.
if [ "$rc" -ne 0 ] && [ "$verdict" -le 1 ]; then
  printf '%s\n' "no-telemetry capture: STOPPED the boot exited $rc before its time (report $report)"
  exit 5
fi
case $verdict in
  0) printf '%s\n' "no-telemetry capture: PASS (report $report)" ;;
  1) printf '%s\n' "no-telemetry capture: FAIL (report $report)" ;;
  3) printf '%s\n' "no-telemetry capture: EMPTY the guest sent nothing (report $report)" ;;
  4) printf '%s\n' "no-telemetry capture: INCOMPLETE no DHCP or no pinned peer (report $report)" ;;
  *) printf '%s\n' "no-telemetry capture: GAP the capture could not be read" ;;
esac
exit "$verdict"

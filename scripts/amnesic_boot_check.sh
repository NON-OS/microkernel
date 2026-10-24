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
# An amnesic boot writes nothing: boot a sealed image as a stick, alone, for
# a while, and compare every byte of the stick and of the firmware's variable
# store afterwards with what went in. Any difference fails, wherever it is:
# a table row, a payload, a log, a crash record, an EFI variable.
#
#   NONOS_AMNESIC_IMAGE    the sealed nonos.img to boot (required)
#   NONOS_AMNESIC_TIMEOUT  seconds the boot runs (default 180)
#   NONOS_AMNESIC_NET      nat (default) or off
#
# OVMF and OVMF_VARS name the firmware, as `nix develop` and `nix run .#qemu`
# set them. The boot is the first boot of a fresh stick: nobody answers
# first-boot setup, so the person has chosen to keep nothing, and the image
# must come back byte for byte. Exit 0 pass, 1 a byte changed, 2 no boot.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"

image=${NONOS_AMNESIC_IMAGE:?set NONOS_AMNESIC_IMAGE to a sealed nonos.img}
timeout_secs=${NONOS_AMNESIC_TIMEOUT:-180}
net=${NONOS_AMNESIC_NET:-nat}
out=target/amnesic
report=$out/report.json
: "${OVMF:?OVMF names the firmware; run inside nix develop}"
: "${OVMF_VARS:?OVMF_VARS names the variable store; run inside nix develop}"

mkdir -p "$out"
rm -f "$report" target/qemu/stick.img target/qemu/OVMF_VARS.fd

sum() { sha256sum "$1" | cut -d' ' -f1; }
image_before=$(sum "$image")
vars_before=$(sum "$OVMF_VARS")

set +e
python3 tools/nonos-qemu --image "$image" --stick --net "$net" --headless \
  --serial "$out/serial.log" --timeout "$timeout_secs"
rc=$?
set -e

if [ ! -f target/qemu/stick.img ] || [ ! -s "$out/serial.log" ]; then
  printf '{"status":"gap","reason":"no boot","exit":%s}\n' "$rc" > "$report"
  printf '%s\n' "amnesic boot: GAP the image did not boot (exit $rc)"
  exit 2
fi

image_after=$(sum target/qemu/stick.img)
vars_after=$(sum target/qemu/OVMF_VARS.fd)
status=pass
[ "$image_before" = "$image_after" ] || status=fail
[ "$vars_before" = "$vars_after" ] || status=fail

changed=""
if [ "$image_before" != "$image_after" ]; then
  changed=$(cmp -l "$image" target/qemu/stick.img | awk '{print int(($1-1)/512)}' | uniq | head -64 | tr '\n' ' ')
fi
printf '{"status":"%s","seconds":%s,"image":{"before":"%s","after":"%s","changed_sectors":"%s"},"efi_vars":{"before":"%s","after":"%s"},"serial_bytes":%s}\n' \
  "$status" "$timeout_secs" "$image_before" "$image_after" "$changed" "$vars_before" "$vars_after" \
  "$(wc -c < "$out/serial.log" | tr -d ' ')" > "$report"
if [ "$status" = pass ]; then
  printf '%s\n' "amnesic boot: PASS the stick and the EFI variables are byte for byte as they went in ($report)"
  exit 0
fi
printf '%s\n' "amnesic boot: FAIL ${changed:+sectors $changed}($report)"
exit 1

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
# An amnesic boot writes nothing: boot a sealed image on its data disk, the
# disk tools/nonos-qemu lays the store on, behind the virtio-blk driver a
# release image brings up under an IOMMU, for a while, and compare every
# byte of that disk afterwards with what went in. The EFI system partition
# QEMU serves from a directory (fat:rw) is compared too, file by file.
# Any difference fails, wherever it is: a table row, a payload, a log, a
# crash record. The firmware's variable store is compared by variable
# (tools/nonos-efi-vars): the firmware fills in its own on a first boot and
# counts every boot, so a variable fails only when it is not the firmware's.
# A second, shorter boot on the same disk, partition and variable store
# follows and must change nothing either, so a write left for shutdown, or
# for the next boot to find, is caught too.
#
#   NONOS_AMNESIC_IMAGE    the sealed nonos.img to boot (required); it is
#                          read, never written
#   NONOS_AMNESIC_TIMEOUT  seconds the boot runs (default 180)
#   NONOS_AMNESIC_SETTLE   seconds the second boot runs (default 120)
#   NONOS_AMNESIC_NET      nat (default) or off
#   NONOS_QEMU_LANES       tools/nonos-qemu lanes, space separated; a release
#                          image refuses device DMA without an IOMMU, so under
#                          QEMU give it `iommu=intel`, or its disk driver never
#                          comes up and the boot cannot write the stick at all
#
# OVMF and OVMF_VARS name the firmware, as `nix develop` and `nix run .#qemu`
# set them. The boot is the first boot of a fresh data disk: nobody answers
# first-boot setup, so the person has chosen to keep nothing, and the disk
# must come back byte for byte. The serial log must show the disk driver up
# and the store answered from the loader's copy of this disk, or the run is a
# gap: a disk the OS could not reach proves nothing. An idle amnesic boot
# reads nothing from the disk itself (the copy answers every read) and keeps
# no disk until a write asks for one, so the disk is reachable, not read: a
# write would select it through the driver checked here, and the byte
# comparison catches it. Exit 0 pass, 1 a byte changed, 2 no boot.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"

image=${NONOS_AMNESIC_IMAGE:?set NONOS_AMNESIC_IMAGE to a sealed nonos.img}
timeout_secs=${NONOS_AMNESIC_TIMEOUT:-180}
settle_secs=${NONOS_AMNESIC_SETTLE:-120}
net=${NONOS_AMNESIC_NET:-nat}
out=target/amnesic
report=$out/report.json
: "${OVMF:?OVMF names the firmware; run inside nix develop}"
: "${OVMF_VARS:?OVMF_VARS names the variable store; run inside nix develop}"

mkdir -p "$out"
rm -f "$report" target/qemu/data.img target/qemu/OVMF_VARS.fd
rm -rf target/qemu/esp
data=target/qemu/data.img

# The data disk as nonos-qemu lays it, laid here so its bytes can be taken
# before the boot; the boot below then finds it and keeps it.
python3 -c 'import sys; sys.path.insert(0, "tools"); from nonos_qemu import disk
disk.prepare(sys.argv[1], "target/qemu/esp", sys.argv[2], True, disk.resolve([], "8G"))
disk.drop_gpt(sys.argv[2])' "$image" "$data"

sum() { sha256sum "$1" | cut -d' ' -f1; }
# Every path under the partition, and every file's bytes, in one digest.
tree_sum() { (cd "$1" && find . -print | LC_ALL=C sort && find . -type f -print0 | LC_ALL=C sort -z | xargs -0 -r sha256sum) | sha256sum | cut -d' ' -f1; }
cp "$data" "$out/data.before"
image_before=$(sum "$data")
esp_before=$(tree_sum target/qemu/esp)
(cd target/qemu/esp && find . -type f -print0 | LC_ALL=C sort -z | xargs -0 -r sha256sum) > "$out/esp.before"
vars_before=$(sum "$OVMF_VARS")

set --
for l in ${NONOS_QEMU_LANES:-}; do set -- "$@" --lane "$l"; done
boot() { # serial log, seconds, then the lanes
  blog=$1
  bsecs=$2
  shift 2
  set +e
  python3 tools/nonos-qemu --image "$image" --net "$net" --headless \
    --serial "$blog" --timeout "$bsecs" "$@"
  rc=$?
  set -e
  if [ ! -s "$blog" ]; then
    printf '{"status":"gap","reason":"no boot","exit":%s}\n' "$rc" > "$report"
    printf '%s\n' "amnesic boot: GAP the image did not boot (exit $rc)"
    exit 2
  fi
  if grep -q 'driver.virtio_blk0 status' "$blog" || ! grep -q 'DRIVER-VIRTIO-BLK\] capsule spawned' "$blog"; then
    printf '{"status":"gap","reason":"the disk driver did not come up"}\n' > "$report"
    printf '%s\n' "amnesic boot: GAP the disk driver did not come up, so nothing could have been written"
    exit 2
  fi
  if ! grep -q "\\[STORE\\] the loader's copy of the store" "$blog"; then
    printf '{"status":"gap","reason":"the store was not answered from this disk"}\n' > "$report"
    printf '%s\n' "amnesic boot: GAP the store was not answered from this disk"
    exit 2
  fi
}
boot "$out/serial.log" "$timeout_secs" "$@"
boot "$out/serial-settle.log" "$settle_secs" "$@"

image_after=$(sum "$data")
esp_after=$(tree_sum target/qemu/esp)
(cd target/qemu/esp && find . -type f -print0 | LC_ALL=C sort -z | xargs -0 -r sha256sum) > "$out/esp.after"
vars_after=$(sum target/qemu/OVMF_VARS.fd)
status=pass
[ "$image_before" = "$image_after" ] || status=fail
[ "$esp_before" = "$esp_after" ] || status=fail
set +e
python3 tools/nonos-efi-vars "$OVMF_VARS" target/qemu/OVMF_VARS.fd --json "$out/efi-vars.json" > "$out/efi-vars.txt"
vars_rc=$?
set -e
[ "$vars_rc" -eq 0 ] || status=fail
foreign=$(grep -c ' FOREIGN ' "$out/efi-vars.txt" || true)

changed=""
if [ "$image_before" != "$image_after" ]; then
  changed=$(cmp -l "$out/data.before" "$data" | awk '{print int(($1-1)/512)}' | uniq | head -64 | tr '\n' ' ')
fi
esp_changed=$(diff "$out/esp.before" "$out/esp.after" | grep '^[<>]' | awk '{print $3}' | sort -u | head -16 | tr '\n' ' ' || true)
printf '{"status":"%s","seconds":%s,"settle_seconds":%s,"image":{"before":"%s","after":"%s","changed_sectors":"%s"},"esp":{"before":"%s","after":"%s","changed_files":"%s"},"efi_vars":{"before":"%s","after":"%s","not_firmware":%s,"report":"%s"},"serial_bytes":%s}\n' \
  "$status" "$timeout_secs" "$settle_secs" "$image_before" "$image_after" "$changed" "$esp_before" "$esp_after" "$esp_changed" \
  "$vars_before" "$vars_after" "$foreign" "$out/efi-vars.json" "$(wc -c < "$out/serial.log" | tr -d ' ')" > "$report"
if [ "$status" = pass ]; then
  printf '%s\n' "amnesic boot: PASS after two boots the data disk is byte for byte as it went in, the system partition file for file, and no EFI variable but the firmware's changed ($report)"
  exit 0
fi
printf '%s\n' "amnesic boot: FAIL ${changed:+sectors $changed}${esp_changed:+, partition files $esp_changed}${foreign:+, $foreign EFI variables not the firmware's }($report)"
exit 1

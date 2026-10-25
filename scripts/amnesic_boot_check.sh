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
# Each is judged right after the main boot, against what went in. A second,
# shorter boot follows on the same disk, partition and variable store, as the
# main boot left them (nonos-qemu --keep-machine), and is judged again, so a
# write left for shutdown, or for the next boot to find, is caught too.
#
#   NONOS_AMNESIC_IMAGE    the sealed nonos.img to boot (required); it is
#                          read, never written
#   NONOS_AMNESIC_TIMEOUT  seconds the boot runs (default 180)
#   NONOS_AMNESIC_SETTLE   seconds the second boot runs (default 120)
#   NONOS_AMNESIC_NET      nat (default) or off
#   NONOS_AMNESIC_QEMU     the runner (default tools/nonos-qemu); the self-test
#                          gives a stand-in (scripts/amnesic_boot_check_selftest.sh)
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
runner=${NONOS_AMNESIC_QEMU:-tools/nonos-qemu}
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
boot() { # serial log, seconds, then nonos-qemu's flags
  blog=$1
  bsecs=$2
  shift 2
  set +e
  python3 "$runner" --image "$image" --net "$net" --headless \
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
status=pass
failed=""
judged=""
# The disk, the partition and the variable store after the boot `$1`, each
# against what went in; a difference fails the run and is named.
judge() {
  tag=$1
  image_now=$(sum "$data")
  esp_now=$(tree_sum target/qemu/esp)
  (cd target/qemu/esp && find . -type f -print0 | LC_ALL=C sort -z | xargs -0 -r sha256sum) > "$out/esp.$tag"
  set +e
  python3 tools/nonos-efi-vars "$OVMF_VARS" target/qemu/OVMF_VARS.fd --json "$out/efi-vars-$tag.json" > "$out/efi-vars-$tag.txt"
  vars_rc=$?
  set -e
  sectors=""
  if [ "$image_now" != "$image_before" ]; then
    status=fail
    sectors=$(cmp -l "$out/data.before" "$data" | awk '{print int(($1-1)/512)}' | uniq | head -64 | tr '\n' ' ')
    failed="$failed $tag: sectors $sectors;"
  fi
  files=$(diff "$out/esp.before" "$out/esp.$tag" | grep '^[<>]' | awk '{print $3}' | sort -u | head -16 | tr '\n' ' ' || true)
  if [ "$esp_now" != "$esp_before" ]; then status=fail; failed="$failed $tag: partition files $files;"; fi
  foreign=$(grep -c ' FOREIGN ' "$out/efi-vars-$tag.txt" || true)
  if [ "$vars_rc" -ne 0 ]; then status=fail; failed="$failed $tag: EFI variables (exit $vars_rc, $foreign not the firmware's);"; fi
  judged="$judged${judged:+,}\"$tag\":{\"image\":\"$image_now\",\"changed_sectors\":\"$sectors\",\"esp\":\"$esp_now\",\"changed_files\":\"$files\",\"efi_vars_exit\":$vars_rc,\"not_firmware\":$foreign,\"vars\":\"$(sum target/qemu/OVMF_VARS.fd)\"}"
}
boot "$out/serial.log" "$timeout_secs" "$@"
judge main
boot "$out/serial-settle.log" "$settle_secs" --keep-machine "$@"
judge settle

printf '{"status":"%s","seconds":%s,"settle_seconds":%s,"before":{"image":"%s","esp":"%s","vars":"%s"},"after":{%s},"serial_bytes":%s}\n' \
  "$status" "$timeout_secs" "$settle_secs" "$image_before" "$esp_before" "$vars_before" "$judged" \
  "$(wc -c < "$out/serial.log" | tr -d ' ')" > "$report"
if [ "$status" = pass ]; then
  printf '%s\n' "amnesic boot: PASS after each of two boots on one machine the data disk is byte for byte as it went in, the system partition file for file, and no EFI variable but the firmware's changed ($report)"
  exit 0
fi
printf '%s\n' "amnesic boot: FAIL$failed ($report)"
exit 1

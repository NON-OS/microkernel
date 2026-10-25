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
# The amnesic check put to a stand-in machine (scripts/amnesic_standin.py),
# which lays the disk, partition and variable store as tools/nonos-qemu does
# and makes one write. A write by the main boot must fail the run under the
# main boot's own name, and again under the settle boot's, which runs on what
# the main boot left; a write by the settle boot fails it too; no write
# passes. Run inside nix develop (OVMF_VARS, mtools). Exit 0 when every case
# holds.
set -eu

ROOT=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$ROOT"
: "${OVMF_VARS:?OVMF_VARS names the variable store; run inside nix develop}"

work=target/amnesic-selftest
rm -rf "$work"
mkdir -p "$work"
img=$work/tiny.img
off=$(python3 -c 'import sys; sys.path.insert(0, "tools"); from nonos_qemu import disk; print(disk.STORE_BYTES)')
truncate -s $((off + 34 * 1024 * 1024)) "$img"
mformat -i "$img@@$off" -T 69632 -h 64 -s 32 -F ::
mmd -i "$img@@$off" ::/EFI
printf 'a loader\n' > "$work/loader"
mcopy -i "$img@@$off" "$work/loader" ::/EFI/BOOTX64.EFI

failed=0
# case: the write, the exit wanted, then each phrase the verdict must hold
case_() {
  write=$1
  want=$2
  shift 2
  set +e
  verdict=$(AMNESIC_WRITE=$write NONOS_AMNESIC_QEMU=scripts/amnesic_standin.py NONOS_AMNESIC_IMAGE=$img \
    NONOS_AMNESIC_NET=off sh scripts/amnesic_boot_check.sh 2>&1 | tail -1)
  rc=$?
  set -e
  rc=$(sed -n 's/.*"status":"\([a-z]*\)".*/\1/p' target/amnesic/report.json)
  ok=1
  [ "$rc" = "$want" ] || ok=0
  for phrase in "$@"; do
    case "$verdict" in *"$phrase"*) ;; *) ok=0 ;; esac
  done
  if [ "$ok" = 1 ]; then
    printf 'ok      %s\n' "$write"
  else
    printf 'FAILED  %s: wanted %s with %s, got %s: %s\n' "$write" "$want" "$*" "$rc" "$verdict"
    failed=1
  fi
}
case_ none pass "PASS"
case_ main:esp fail "main: partition files ./EFI/crash.txt" "settle: partition files ./EFI/crash.txt"
case_ main:disk fail "main: sectors 300" "settle: sectors 300"
case_ main:vars fail "main: EFI variables" "settle: EFI variables"
case_ settle:esp fail "settle: partition files ./EFI/crash.txt"
exit "$failed"

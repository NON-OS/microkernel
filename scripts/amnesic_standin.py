#!/usr/bin/env python3
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
"""A stand-in for tools/nonos-qemu, for scripts/amnesic_boot_check_selftest.sh.
It lays the machine as nonos-qemu does, with nonos-qemu's own disk.prepare and
disk.vars_store (--keep-machine included), writes the serial lines the check
requires of a boot, and makes the one write AMNESIC_WRITE names for this boot:
main:esp, main:disk, main:vars, settle:esp, or none. A fresh boot's store is
the one a real first boot left (testdata/ovmf-vars-first-boot.fd.gz, from
faultproof-dev under this flake's OVMF): firmware fills the store on a first
boot, and the check takes an untouched store for no boot at all."""

import argparse
import gzip
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "tools"))
from nonos_qemu import disk  # noqa: E402

TARGET = "target/qemu"
# EFI_GLOBAL_VARIABLE, 8be4df61-93ca-11d2-aa0d-00e098032b8c, as a store holds it.
GLOBAL = bytes.fromhex("61dfe48bca93d211aa0d00e098032b8c")
FIRST_BOOT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "testdata", "ovmf-vars-first-boot.fd.gz")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--image", required=True)
    ap.add_argument("--serial", required=True)
    ap.add_argument("--keep-machine", action="store_true")
    a, _ = ap.parse_known_args()
    esp, data = os.path.join(TARGET, "esp"), os.path.join(TARGET, "data.img")
    disk.prepare(a.image, esp, data, False, [], a.keep_machine)
    store = disk.vars_store(TARGET, os.environ["OVMF_VARS"], a.keep_machine)
    if not a.keep_machine:
        with gzip.open(FIRST_BOOT) as src, open(store, "wb") as dst:
            dst.write(src.read())
    with open(a.serial, "w") as f:
        f.write("[DRIVER-VIRTIO-BLK] capsule spawned\n[STORE] the loader's copy of the store\n")
    boot = "settle" if a.keep_machine else "main"
    write = os.environ.get("AMNESIC_WRITE", "none")
    if write == f"{boot}:esp":
        with open(os.path.join(esp, "EFI", "crash.txt"), "w") as f:
            f.write("a crash record\n")
    elif write == f"{boot}:disk":
        with open(data, "r+b") as f:
            f.seek(300 * 512)
            f.write(b"x")
    elif write == f"{boot}:vars":
        # A variable under a GUID the firmware never writes: a live EFI
        # global variable's GUID, one byte changed in place. A live record of
        # the authenticated store: StartId 0x55AA, state VAR_ADDED (0x3F),
        # the vendor GUID 44 bytes in.
        with open(store, "r+b") as f:
            body = bytearray(f.read())
            at = next(i + 44 for i in range(len(body) - 60)
                      if body[i:i + 3] == b"\xaa\x55\x3f" and body[i + 44:i + 60] == GLOBAL)
            body[at] ^= 0x01
            f.seek(0)
            f.write(body)
    return 0


if __name__ == "__main__":
    sys.exit(main())

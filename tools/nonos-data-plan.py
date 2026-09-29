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
"""Lay out a disk for the data volume: the plan sector, and a file to import.

The kernel reads one plain sector at LBA 65536, the disk plan, to find its
data volume and a file waiting to be imported (src/fs/blockfs_volume/
plan_types.rs has the layout). This writes that sector into a raw image,
copies the file to import past the volume, and grows the image sparsely to
hold both. The plan is untrusted by design: the kernel checks every range,
and keeps an import only if it hashes to the digest a signed capsule pins.

    nonos-data-plan.py IMAGE --volume-sectors N [--import FILE] [--fresh]

--fresh zeroes the volume's 256-sector header ring, so the next boot formats
a new volume instead of opening the last one.
"""

import argparse
import os
import struct

SECTOR = 512
PLAN_LBA = 65_536
DATA_FLOOR = 131_072
RING = 256


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("image")
    ap.add_argument("--volume-sectors", type=int, required=True)
    ap.add_argument("--import", dest="blob")
    ap.add_argument("--fresh", action="store_true")
    a = ap.parse_args()
    base = DATA_FLOOR
    import_lba = base + a.volume_sectors
    blob = os.path.getsize(a.blob) if a.blob else 0
    end = import_lba + (blob + SECTOR - 1) // SECTOR
    plan = b"NONOSDP1" + struct.pack("<4Q", base, a.volume_sectors, import_lba if blob else 0, blob)
    with open(a.image, "r+b") as img:
        if os.path.getsize(a.image) < end * SECTOR:
            img.truncate(end * SECTOR)
        img.seek(PLAN_LBA * SECTOR)
        img.write(plan.ljust(SECTOR, b"\0"))
        if a.fresh:
            img.seek(base * SECTOR)
            img.write(b"\0" * RING * SECTOR)
        if blob:
            img.seek(import_lba * SECTOR)
            with open(a.blob, "rb") as src:
                while chunk := src.read(1 << 20):
                    img.write(chunk)
    print(f"plan at LBA {PLAN_LBA}: volume {a.volume_sectors} sectors at {base}, "
          f"import {blob} bytes at {import_lba if blob else 0}; image {end * SECTOR} bytes")


if __name__ == "__main__":
    main()

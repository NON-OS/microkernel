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
"""The command line: size the layout, write it, say where everything went."""

import argparse
import os
import struct
import sys

from .image import PLAN_LBA, SECTOR, kept_sectors, write, write_live

DATA_FLOOR = 262_144
MAX_IMPORTS = 30


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("image")
    ap.add_argument("--volume-sectors", type=int)
    # A live stick's plan: no volume on the disk, the files to import laid
    # from this LBA on (the release stick puts them past its ESP).
    ap.add_argument("--live-at", type=int)
    ap.add_argument("--import", dest="blobs", action="append", default=[])
    ap.add_argument("--fresh", action="store_true")
    a = ap.parse_args()
    if (a.volume_sectors is None) == (a.live_at is None):
        ap.error("give --volume-sectors, or --live-at for a live stick")
    if a.live_at is not None:
        live(a)
        return
    base = DATA_FLOOR
    if len(a.blobs) > MAX_IMPORTS:
        ap.error(f"at most {MAX_IMPORTS} imports fit in the plan sector")
    if not a.fresh:
        kept = kept_sectors(a.image, base)
        if kept > a.volume_sectors:
            print(f"keeping the volume at {kept} sectors, as it was formatted; "
                  f"--fresh makes a new one of {a.volume_sectors}")
            a.volume_sectors = kept
    entries, end = [], base + a.volume_sectors
    for path in a.blobs:
        size = os.path.getsize(path)
        if size == 0:
            ap.error(f"{path} is empty")
        entries.append((path, end, size))
        end += (size + SECTOR - 1) // SECTOR
    plan = b"NONOSDP1" + struct.pack("<3Q", base, a.volume_sectors, len(entries))
    plan += b"".join(struct.pack("<2Q", lba, size) for _, lba, size in entries)
    write(a.image, plan, entries, end, base, a.fresh)
    print(f"plan at LBA {PLAN_LBA}: volume {a.volume_sectors} sectors at {base}; "
          f"image {end * SECTOR} bytes")
    for path, lba, size in entries:
        print(f"  import {os.path.basename(path)}: {size} bytes at LBA {lba}")


def live(a):
    if a.live_at < DATA_FLOOR or not a.blobs:
        sys.exit(f"--live-at needs files to import and an LBA of at least {DATA_FLOOR}")
    entries, end = [], a.live_at
    for path in a.blobs:
        size = os.path.getsize(path)
        if size == 0:
            sys.exit(f"{path} is empty")
        entries.append((path, end, size))
        end += (size + SECTOR - 1) // SECTOR
    plan = b"NONOSDP1" + struct.pack("<3Q", 0, 0, len(entries))
    plan += b"".join(struct.pack("<2Q", lba, size) for _, lba, size in entries)
    write_live(a.image, plan, entries, end)
    print(f"live plan at LBA {PLAN_LBA}: no volume; image {end * SECTOR} bytes")
    for path, lba, size in entries:
        print(f"  import {os.path.basename(path)}: {size} bytes at LBA {lba}")


if __name__ == "__main__":
    main()

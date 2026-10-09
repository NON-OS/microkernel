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
"""Write the plan sector and the imports into the image or disk."""

import os
import stat
import struct
import sys

SECTOR = 512
PLAN_LBA = 245_760
# The key header, the sector after the plan: how the volume key is reached.
KEY_LBA = PLAN_LBA + 1
RING = 256
GPT_SIGNATURE = b"EFI PART"


def backup_gpt(img):
    """The sectors the primary GPT at LBA 1 says its backup lies in (the
    entries, then the header at its alternate LBA), as [first, end), or None
    when the disk carries no GPT. Firmware that finds the backup damaged
    writes it back there at every boot, whatever was written over it."""
    img.seek(SECTOR)
    header = img.read(SECTOR)
    if len(header) < 92 or header[:8] != GPT_SIGNATURE:
        return None
    alternate, last_usable = struct.unpack_from("<Q8xQ", header, 32)
    count, size = struct.unpack_from("<2I", header, 80)
    entries = (count * size + SECTOR - 1) // SECTOR
    return min(alternate - entries, last_usable + 1), alternate + 1


def clear_gpt(image, first, end):
    """Remove a partition table whose backup lies inside [first, end), the
    data volume and its imports, so no firmware writes it back over them.
    The sealed stick's table comes along when the store region is copied
    from it, and it says the disk ends at the stick's 384 MiB, inside the
    volume. Only an image file is changed; a real disk with such a table is
    refused. The protective MBR, the header and the entries go. Whether one
    was removed."""
    with open(image, "r+b") as img:
        backup = backup_gpt(img)
        if backup is None or not (backup[0] < end and first < backup[1]):
            return False
        if stat.S_ISBLK(os.fstat(img.fileno()).st_mode):
            sys.exit(f"{image}: its partition table keeps a backup at LBA {backup[0]} to "
                     f"{backup[1] - 1}, inside the data volume, and firmware would write it "
                     "back over the volume at every boot; refused")
        img.seek(SECTOR)
        header = img.read(SECTOR)
        entries_lba = struct.unpack_from("<Q", header, 72)[0]
        count, size = struct.unpack_from("<2I", header, 80)
        last = max(1, entries_lba + (count * size + SECTOR - 1) // SECTOR - 1)
        img.seek(0)
        img.write(b"\0" * (last + 1) * SECTOR)
        return True


def kept_sectors(image, base):
    """The volume the image's plan already gives at `base`, or 0. A volume is
    formatted to its plan's size, and a later plan must not give it less: the
    blocks past the smaller window could not be read, and imports placed there
    would overwrite them."""
    try:
        with open(image, "rb") as img:
            img.seek(PLAN_LBA * SECTOR)
            sector = img.read(SECTOR)
    except OSError:
        return 0
    if len(sector) < 32 or sector[:8] != b"NONOSDP1":
        return 0
    old_base, old_sectors = struct.unpack_from("<2Q", sector, 8)
    return old_sectors if old_base == base else 0


def write(image, plan, entries, end, base, fresh):
    """Put `plan` at PLAN_LBA and each import at its LBA; with `fresh`, zero
    the volume's header ring at `base` and the key header at KEY_LBA. `end`
    is the first sector past it all."""
    if clear_gpt(image, base, end):
        print(f"{image}: removed the partition table copied with the store; its backup lay "
              "inside the data volume, where firmware writes it back at every boot")
    with open(image, "r+b") as img:
        # A disk is sized by seeking to its end and cannot grow; an image
        # file grows sparsely to hold the plan.
        size = img.seek(0, os.SEEK_END)
        if stat.S_ISBLK(os.fstat(img.fileno()).st_mode):
            if size < end * SECTOR:
                sys.exit(f"{image}: {size} bytes, the plan needs {end * SECTOR}")
        elif size < end * SECTOR:
            img.truncate(end * SECTOR)
        img.seek(PLAN_LBA * SECTOR)
        img.write(plan.ljust(SECTOR, b"\0"))
        if fresh:
            img.seek(base * SECTOR)
            img.write(b"\0" * RING * SECTOR)
            img.seek(KEY_LBA * SECTOR)
            img.write(b"\0" * SECTOR)
        for path, lba, _ in entries:
            img.seek(lba * SECTOR)
            with open(path, "rb") as src:
                while chunk := src.read(1 << 20):
                    img.write(chunk)


def write_live(image, plan, entries, end):
    """A live stick's plan at PLAN_LBA and each import at its LBA. Nothing
    else on the stick is touched: its partition table and ESP stay, and the
    imports lie past them."""
    with open(image, "r+b") as img:
        size = img.seek(0, os.SEEK_END)
        if stat.S_ISBLK(os.fstat(img.fileno()).st_mode):
            if size < end * SECTOR:
                sys.exit(f"{image}: {size} bytes, the plan needs {end * SECTOR}")
        elif size < end * SECTOR:
            img.truncate(end * SECTOR)
        img.seek(PLAN_LBA * SECTOR)
        img.write(plan.ljust(SECTOR, b"\0"))
        for path, lba, _ in entries:
            img.seek(lba * SECTOR)
            with open(path, "rb") as src:
                while chunk := src.read(1 << 20):
                    img.write(chunk)

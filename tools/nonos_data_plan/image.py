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
import sys

SECTOR = 512
PLAN_LBA = 65_536
# The key header, the sector after the plan: how the volume key is reached.
KEY_LBA = PLAN_LBA + 1
RING = 256


def write(image, plan, entries, end, base, fresh):
    """Put `plan` at PLAN_LBA and each import at its LBA; with `fresh`, zero
    the volume's header ring at `base` and the key header at KEY_LBA. `end`
    is the first sector past it all."""
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

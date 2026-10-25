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

"""What the first boot left behind: on the disk it booted from, and in what
the second boot printed."""

from pathlib import Path

from .memscan import count

SECTOR = 512
SLICE = 64 << 20


def changed_sectors(before_image, after_image):
    """Each run of sectors the boot changed on its disk, as (first, count).
    A boot that keeps nothing changes none; the whole disk is compared, not
    only the store's table."""
    runs, sector = [], 0
    with open(before_image, "rb") as x, open(after_image, "rb") as y:
        while True:
            a, b = x.read(SLICE), y.read(SLICE)
            if not a and not b:
                return runs
            for i in range(0, max(len(a), len(b)), SECTOR):
                if a[i:i + SECTOR] != b[i:i + SECTOR]:
                    if runs and runs[-1][0] + runs[-1][1] == sector:
                        runs[-1] = (runs[-1][0], runs[-1][1] + 1)
                    else:
                        runs.append((sector, 1))
                sector += 1


def disk_holds(image, secret):
    """How many times the secret is anywhere on the disk, sector by sector."""
    return count(image, secret)


def said(serial, secret):
    return secret in Path(serial).read_text(errors="replace")

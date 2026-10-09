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
"""`plan`: check a tier's files against their pins and lay them on a disk."""

import os
import subprocess
import sys

from .pins import TOOLS, check
from .upstream import upstream

# 484 plain bytes per sealed 512-byte sector, one pointer block per 60, a margin.
PLAIN, FANOUT, SPARE = 484, 60, 65_536


def volume_sectors(sizes):
    data = sum(-(-s // PLAIN) for s in sizes)
    return data + data // (FANOUT - 1) + SPARE


def plan(files, into, image, fresh):
    # Fetched under the names they are published by; the volume matches
    # each by its length and SHA-256, not its name.
    files = [(upstream(n), s, d) for n, s, d in files]
    for name, size, digest in files:
        if not check(os.path.join(into, name), size, digest):
            sys.exit(f"{name}: missing, or its length or SHA-256 differs from the pin")
    cmd = [sys.executable, os.path.join(TOOLS, "nonos-data-plan.py"), image,
           "--volume-sectors", str(volume_sectors([s for _, s, _ in files]))]
    cmd += [x for n, _, _ in files for x in ("--import", os.path.join(into, n))]
    sys.exit(subprocess.call(cmd + (["--fresh"] if fresh else [])))

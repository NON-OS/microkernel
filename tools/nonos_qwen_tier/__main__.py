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
"""The command line: list, fetch and plan."""

import argparse
import os
import subprocess
import sys

from .download import fetch
from .pins import TOOLS, check, pins

# 484 plain bytes per sealed 512-byte sector, one pointer block per 60, a margin.
PLAIN, FANOUT, SPARE = 484, 60, 65_536


def volume_sectors(sizes):
    data = sum(-(-s // PLAIN) for s in sizes)
    return data + data // (FANOUT - 1) + SPARE


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("verb", choices=["list", "fetch", "plan"])
    ap.add_argument("tiers", nargs="*")
    ap.add_argument("--dir", default=".")
    ap.add_argument("--image")
    ap.add_argument("--fresh", action="store_true")
    a, table = ap.parse_args(), pins()
    if a.verb == "list":
        for tier, files in table.items():
            print(f"{tier:13} {sum(s for _, s, _ in files) / 1e9:5.2f} GB  "
                  + " ".join(n for n, _, _ in files))
        return
    unknown = [t for t in a.tiers if t not in table]
    if not a.tiers or unknown:
        ap.error(f"name one or more tiers of: {' '.join(table)}")
    files = [f for t in dict.fromkeys(a.tiers) for f in table[t]]
    if a.verb == "fetch":
        os.makedirs(a.dir, exist_ok=True)
        for name, size, digest in files:
            fetch(name, size, digest, a.dir)
        return
    if not a.image:
        ap.error("plan needs --image (a raw image file or a whole disk)")
    for name, size, digest in files:
        if not check(os.path.join(a.dir, name), size, digest):
            sys.exit(f"{name}: missing, or its length or SHA-256 differs from the pin")
    cmd = [sys.executable, os.path.join(TOOLS, "nonos-data-plan.py"), a.image,
           "--volume-sectors", str(volume_sectors([s for _, s, _ in files]))]
    cmd += [x for n, _, _ in files for x in ("--import", os.path.join(a.dir, n))]
    sys.exit(subprocess.call(cmd + (["--fresh"] if a.fresh else [])))


if __name__ == "__main__":
    main()

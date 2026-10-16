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

"""The proof crates that mount any of the given source files.

A proof crate includes shipped source by `#[path = "..."]` or `include!`,
and the static gate builds none of them, so a change to a mounted file can
leave a crate that no longer compiles. Name the files (or a git range with
--since REV) and this prints each crate directory that mounts one, so the
author runs exactly those. Directory mounts count for every file below them.
"""

import argparse
import os
import re
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
MOUNT = re.compile(r'(?:#\[path\s*=\s*|include(?:_str)?!\s*\(\s*)"([^"]+)"')


def crates():
    for top in ("userland", "proofs", "verification"):
        base = os.path.join(ROOT, top)
        for dirpath, dirnames, filenames in os.walk(base):
            dirnames[:] = [d for d in dirnames if d not in ("target", ".git")]
            if "Cargo.toml" in filenames and os.path.isdir(os.path.join(dirpath, "src")):
                yield dirpath


def mounts(crate):
    for dirpath, _, filenames in os.walk(os.path.join(crate, "src")):
        for name in filenames:
            if not name.endswith(".rs"):
                continue
            src = os.path.join(dirpath, name)
            with open(src, encoding="utf-8", errors="replace") as f:
                for target in MOUNT.findall(f.read()):
                    yield os.path.normpath(os.path.join(dirpath, target))


def changed(since):
    out = subprocess.run(["git", "-C", ROOT, "diff", "--name-only", since, "--"],
                         check=True, capture_output=True, text=True).stdout
    return [os.path.join(ROOT, p) for p in out.split()]


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("files", nargs="*")
    ap.add_argument("--since", help="every file changed since this revision")
    args = ap.parse_args()
    wanted = [os.path.normpath(os.path.abspath(f)) for f in args.files]
    if args.since:
        wanted += changed(args.since)
    if not wanted:
        ap.error("name files or --since REV")
    hits = set()
    for crate in crates():
        for m in mounts(crate):
            if any(w == m or w.startswith(m + os.sep) for w in wanted):
                hits.add(os.path.relpath(crate, ROOT))
                break
    for crate in sorted(hits):
        print(crate)
    return 0


if __name__ == "__main__":
    sys.exit(main())

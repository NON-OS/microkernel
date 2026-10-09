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
"""Every committed binary is classified, and every upstream one matches its pinned hash."""

import argparse
import fnmatch
import hashlib
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
RULES, PINS = (ROOT / "scripts/baselines" / n for n in ("prebuilt.txt", "upstream.sha256"))
EXTS = (".bin", ".der", ".pem", ".pub", ".efi", ".img", ".elf", ".ucode", ".o", ".a", ".so", ".wasm")


def lines(path):
    return [l.split() for l in path.read_text().splitlines() if l.strip() and l[0] != "#"]


def sha(name):
    return hashlib.sha256((ROOT / name).read_bytes()).hexdigest()


def is_binary(name):
    with open(ROOT / name, "rb") as f:
        return name.endswith(EXTS) or f.read(4).startswith((b"\x7fELF", b"MZ"))


def classified():
    table = lines(RULES)
    ls = subprocess.run(["git", "ls-files"], cwd=ROOT, capture_output=True, text=True)
    for name in ls.stdout.splitlines():
        if (ROOT / name).is_file() and is_binary(name):
            yield name, next((c for c, pat in table if fnmatch.fnmatch(name, pat)), None)


def problems(pins):
    counts = {}
    for name, cls in classified():
        counts[cls] = counts.get(cls, 0) + 1
        if cls is None:
            yield f"unclassified binary {name}: build it from source or add a rule"
        elif cls == "upstream" and pins.get(name) != sha(name):
            yield f"upstream {name} is not pinned at sha256 {sha(name)}"
    print("".join(f"prebuilt: {counts[c]:4d} {c}\n" for c in sorted(counts, key=str)), end="")


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--pin", action="store_true", help="print sha256 lines for every upstream file")
    if ap.parse_args().pin:
        print("".join(f"{sha(n)} {n}\n" for n, c in classified() if c == "upstream"), end="")
        return 0
    found = list(problems({n: d for d, n in lines(PINS)}))
    print("".join(f"prebuilt: {p}\n" for p in found) + f"prebuilt: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())

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
"""Fail when nonos_blk_client's per-driver table drifts from a block driver.

Each row of the client's table (magic, four ops, service name) is held to the
capsule whose Capsule.mk registers that name, by the constants its protocol
files define, or take from nonos_blk_wire, read by name: a row added to the
table is checked as it is."""

import argparse
import sys
from pathlib import Path

from blk_wire import CLIENT, WANT, consts, handles, protocol, table
from blk_wire_selftest import self_test


def problems(root):
    wire, rows, owners, out = consts((root / CLIENT / "wire").glob("*.rs")), table(root), handles(root), []
    for d, row in sorted(rows.items()):
        crate = owners.get(row.get("service"))
        theirs = protocol(root, crate) if crate else {}
        for key, names in WANT.items():
            ours = row.get(key, wire.get(key))
            got = next((theirs[n] for n in names.split() if n in theirs), None)
            if ours is None or ours != got:
                who = crate.name if crate else f"no capsule registering {row.get('service')}"
                out.append(f"{d} {key}: the client says {ours}, {who} {names} says {got}")
    return out, len(rows)


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--self-test", action="store_true")
    a = ap.parse_args()
    if a.self_test:
        return self_test(problems) or 0
    found, n = problems(a.root)
    print("".join(f"blk wire: {p}\n" for p in found) + f"blk wire: {n} drivers, {len(found)} differ")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())

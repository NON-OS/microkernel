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

"""Check that a published syscall number, errno or capability bit never changes
meaning: against the baseline, nothing is renumbered or withdrawn, no two names
share a value, and a new entry is published on purpose, with --write."""

import argparse
import pathlib
import sys
import tomllib

ROOT = pathlib.Path(__file__).resolve().parent.parent
BASELINE = ROOT / "scripts/baselines/abi-published.txt"
TABLES = [("syscalls", "numbers"), ("syscalls", "errors"), ("caps", "bits")]


def published():
    out = {}
    for doc, table in TABLES:
        data = tomllib.loads((ROOT / f"abi/{doc}.toml").read_text())
        for name, value in data[table].items():
            out[(f"{doc}.{table}", name)] = int(value)
    return out


def problems(now, base):
    for key, value in sorted(base.items()):
        if key not in now:
            yield f"withdrawn: {key[0]} {key[1]} (was {value})"
        elif now[key] != value:
            yield f"renumbered: {key[0]} {key[1]} {value} -> {now[key]}"
    for key in sorted(set(now) - set(base)):
        yield f"not in the baseline: {key[0]} {key[1]} = {now[key]} (publish with --write)"
    seen = {}
    for (table, name), value in sorted(now.items()):
        if (table, value) in seen:
            yield f"shared value: {table} {seen[(table, value)]} and {name} are both {value}"
        seen[(table, value)] = name


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--write", action="store_true", help="publish the current tables")
    args = ap.parse_args()
    now = published()
    if args.write:
        lines = [f"{t} {n} {v}" for (t, n), v in sorted(now.items())]
        BASELINE.write_text("\n".join(lines) + "\n")
        print(f"abi-stable: wrote {len(lines)} published entries")
        return 0
    rows = (line.split() for line in BASELINE.read_text().splitlines())
    base = {(table, name): int(value) for table, name, value in rows}
    found = list(problems(now, base))
    print("".join(f"abi-stable: {p}\n" for p in found), end="", file=sys.stderr)
    print(f"abi-stable: {len(now)} published entries, {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())

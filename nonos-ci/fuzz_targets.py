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
"""The fuzz job's matrix, from fuzz/targets.json held to the tree.

Every [[bin]] of every tracked */fuzz/Cargo.toml must be named in
fuzz/targets.json, under "ci" (fuzzed by the workflow) or "not_in_ci", and
every name there must still be a target, so the list cannot drift from the
tree either way. Prints the matrix as JSON ({"include": [...]}) and, under
Actions, writes it to $GITHUB_OUTPUT as `matrix` and the targets left out to
the job summary.

    nonos-ci/fuzz_targets.py [--check]
"""

import json
import os
import subprocess
import sys
import tomllib

LIST = "fuzz/targets.json"


def tree_targets():
    """(crate, target) for every fuzz binary in a tracked fuzz/Cargo.toml."""
    out = set()
    files = subprocess.run(["git", "ls-files", "*fuzz/Cargo.toml"], check=True,
                           capture_output=True, text=True).stdout.split()
    for path in files:
        crate = os.path.dirname(os.path.dirname(path))
        with open(path, "rb") as src:
            for b in tomllib.load(src).get("bin", []):
                out.add((crate, b["name"]))
    return out


def slug(crate, target):
    return f"{crate.replace('/', '-')}-{target}"


def main():
    os.chdir(os.path.join(os.path.dirname(os.path.abspath(__file__)), ".."))
    with open(LIST, encoding="utf-8") as src:
        listed = json.load(src)
    ci = [(t["crate"], t["target"]) for t in listed["ci"]]
    left = [(t["crate"], t["target"]) for t in listed["not_in_ci"]]
    tree = tree_targets()
    errors = []
    seen = set()
    for pair in ci + left:
        if pair in seen:
            errors.append(f"{pair[0]} {pair[1]}: named twice in {LIST}")
        seen.add(pair)
        if pair not in tree:
            errors.append(f"{pair[0]} {pair[1]}: named in {LIST} but no fuzz/Cargo.toml has it")
    for pair in sorted(tree - seen):
        errors.append(f"{pair[0]} {pair[1]}: a fuzz target {LIST} does not name (add it to ci or not_in_ci)")
    if errors:
        for e in errors:
            print(f"::error file={LIST}::{e}", file=sys.stderr)
        return 1
    if "--check" in sys.argv:
        print(f"fuzz_targets: {len(ci)} fuzzed in CI, {len(left)} not, {len(tree)} in the tree")
        return 0

    matrix = {"include": [
        {"crate": t["crate"], "target": t["target"], "seed": bool(t.get("seed")),
         "slug": slug(t["crate"], t["target"])}
        for t in listed["ci"]]}
    text = json.dumps(matrix, separators=(",", ":"))
    print(text)
    if os.environ.get("GITHUB_OUTPUT"):
        with open(os.environ["GITHUB_OUTPUT"], "a", encoding="utf-8") as out:
            out.write(f"matrix={text}\n")
    if os.environ.get("GITHUB_STEP_SUMMARY"):
        with open(os.environ["GITHUB_STEP_SUMMARY"], "a", encoding="utf-8") as out:
            out.write(f"{len(ci)} fuzz targets run here; {len(left)} are not fuzzed in CI ({LIST}):\n\n")
            out.write("".join(f"- `{c}` `{t}`\n" for c, t in left))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

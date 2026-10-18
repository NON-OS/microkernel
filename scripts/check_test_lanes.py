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
"""Every crate's tests run in a lane, or the crate is listed with its owner.

The lane is `nix flake check`'s proofs-<name>: cargo test in the crate's
directory against its own Cargo.lock. tools/nix/checks.nix runs it for every
userland/*_proofs crate and for every crate whose manifest asks:

    [package.metadata.nonos]
    test-lane = true
    test-args = "--workspace"    # optional, appended to cargo test

tools/nix/test-lanes.json is those markers as nix and tools/nix/inputs.py read
them; --write-lanes writes it, and the check fails when it is stale.

A crate with a test fn, a proptest! or a program under tests/ that no lane
runs fails, unless scripts/baselines/untested-crates.txt lists it. So does a
test file in a lane crate that no target reaches, or that its test-args skip.
A listed path that no longer needs its line fails too, so the list only
shrinks. It reads the tree; it never runs cargo.
"""

import argparse
import sys
from pathlib import Path

from test_lanes.baseline import LANES, render
from test_lanes.lanes import marked
from test_lanes.selftest import self_test
from test_lanes.tree import Tree
from test_lanes.verdict import verdict


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--root", type=Path, default=Path("."))
    ap.add_argument("--write-lanes", action="store_true", help=f"write {LANES} from the markers")
    ap.add_argument("--self-test", action="store_true", help="prove every rule bites on fixture trees")
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    tree = Tree(args.root)
    if args.write_lanes:
        (args.root / LANES).write_text(render(marked(tree)))
        print(f"test-lanes: {LANES} written, {len(marked(tree))} marked crates")
    problems, lane, listed = verdict(tree)
    for p in problems:
        print(f"test-lanes: {p}")
    files = sum(1 for p in listed if p not in tree.manifests)
    print(f"test-lanes: {len(lane)} crates in a lane, {len(listed) - files} crates and {files} "
          f"files listed, {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())

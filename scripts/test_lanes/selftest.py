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
"""Each rule on the fixture: the tree that passes, and one edit per way to
fail, each of which must give exactly the problems it names."""

import tempfile
from pathlib import Path

from .fixture import BL, LISTED, LJ, MARK, PKG, TEST, build, skipping, workspace
from .tree import Tree
from .verdict import verdict

LOST = "userland/x_proofs/src/lost.rs"
# The marked crate asking for its tests again with a feature on.
FEATURED = PKG.format("marked") + MARK + 'feature-lanes = ["devnet"]\n'
DECLARED = FEATURED + "\n[features]\ndevnet = []\n"
WITH = '{\n "marked": "",\n "marked+devnet": "--features devnet"\n}\n'
# (case, edits, the text each expected problem holds; none for a pass)
CASES = [
    ("the fixture as built", {}, []),
    ("an unmarked crate off the list", {BL: ""}, ["unmarked has tests in no lane"]),
    ("a program under tests/", {"tool/Cargo.toml": '[package]\nname = "tool"\n',
                                "tool/tests/host/run.rs": "fn main() {}\n"}, ["tool has tests in no lane"]),
    ("a capsule test with its mount gone", {"userland/x_proofs/src/lib.rs": "mod local;\n"},
     ["userland/capsule has tests in no lane"]),
    ("a stale line, no tests", {BL: LISTED + "helpers a lead x\n"}, ["helpers, which is stale: it has no"]),
    ("a stale line, in a lane", {BL: LISTED + "marked a lead x\n"}, ["marked, which is stale: it is in a"]),
    ("a stale line, gone", {BL: LISTED + "gone a lead x\n"}, ["gone, which is stale: it is gone"]),
    ("an orphan", {LOST: TEST}, [f"{LOST} holds tests that no target"]),
    ("an orphan on the list", {LOST: TEST, BL: LISTED + f"{LOST} orphan A x\n"}, []),
    ("an orphan listed as a crate", {LOST: TEST, BL: LISTED + f"{LOST} a A x\n"}, ["class a, but it is an orphan"]),
    ("a dropped target", skipping("--lib --test kept"), ["skipped.rs holds tests of marked that its test-args"]),
    ("a dropped target on the list",
     {**skipping("--lib --test kept"), BL: LISTED + "marked/tests/skipped.rs c lead x\n"}, []),
    ("test-args that select every target", skipping("--tests"), []),
    ("test-lanes.json off the markers", {LJ: '{\n "marked": "--lib"\n}\n'}, ["gives marked test-args '--lib'"]),
    ("test-lanes.json missing a crate", {LJ: "{}\n"}, ["test-lanes.json lacks marked"]),
    ("test-lanes.json laid out by hand", {LJ: '{"marked": ""}\n'}, ["not byte for byte"]),
    ("a marker with no lock", {"marked/Cargo.lock": None}, ["has none", "marked has tests in no lane"]),
    ("a malformed line", {BL: "unmarked z lead x\n"}, ["class 'z'", "unmarked has tests in no lane"]),
    ("a member its root runs", workspace("--workspace"), []),
    ("a member its root leaves out", workspace(""), ["ws/core has tests in no lane"]),
    ("a feature lane", {"marked/Cargo.toml": DECLARED, LJ: WITH}, []),
    ("a feature lane test-lanes.json lacks", {"marked/Cargo.toml": DECLARED}, ["test-lanes.json lacks marked+devnet"]),
    ("an undeclared feature's lane", {"marked/Cargo.toml": FEATURED, LJ: WITH}, ["which [features] does not declare"]),
]


def self_test():
    bad = 0
    for name, edits, want in CASES:
        with tempfile.TemporaryDirectory() as d:
            build(Path(d), edits)
            got = verdict(Tree(Path(d)))[0]
        if len(got) != len(want) or not all(any(w in p for p in got) for w in want):
            bad += 1
            print(f"test-lanes: self-test case '{name}' wanted {want}, got {got}")
    if bad:
        print(f"test-lanes: self-test failed, {bad} of {len(CASES)} cases misbehaved")
        return 1
    print(f"test-lanes: self-test passed, all {len(CASES)} cases behaved")
    return 0

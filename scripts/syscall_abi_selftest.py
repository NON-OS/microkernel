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
"""The retired-number rules on fixtures: a retired number that reaches the
gate passes, and each way it can stop reaching it gives exactly the problem
it names; and every arm shape counts each name it matches on, so an arm left
behind for a retired number is seen wherever it sits in an or-pattern. Read
by check_syscall_abi --self-test."""

import tempfile
from pathlib import Path

from syscall_mk_arms import mk_arms, mk_retired

TAG, VARIANT, CONST = "MXRT", "MkRetiredFixture", "SYS_RETIRED_FIXTURE"
DISPATCH = Path("src/syscall/microkernel/dispatch")
RETIRED = Path("src/syscall/microkernel/retired.rs")
GATE = "    if let Some(r) = crate::syscall::microkernel::retired::handle(nr) {\n"
TABLE = 'pub(crate) const RETIRED: [[u8; 4]; 2] = [*b"MXRT", *b"MXRU"];\n'


def tag4(tag):
    b = tag.encode()
    return b[0] | (b[1] << 8) | (b[2] << 16) | (b[3] << 24)


def kernel(**edits):
    """The fixture kernel: the number registered, capped, routed to the
    microkernel, retired, and with no arm left for it."""
    k = {"enum": {TAG: VARIANT}, "registry": {TAG: VARIANT},
         "mk_consts": {TAG: CONST}, "capped": {VARIANT}, "routed": {VARIANT},
         "mk_routed": {VARIANT}, "mk_arms": set(), "mk_retired": {TAG}}
    k.update(edits)
    return k


# (case, edits, the text each expected problem holds; none for a pass)
CASES = [
    ("a retired number at the gate", {}, []),
    ("the same number, not retired", {"mk_retired": set()}, ["has no dispatch arm"]),
    ("an arm left for it", {"mk_arms": {CONST}}, ["still has a dispatch arm"]),
    ("routed elsewhere", {"mk_routed": set()}, ["routes to a family other than"]),
    ("in no cap-table family", {"capped": set()}, ["is in no cap-table family"]),
]


def tree(route, table):
    """A fixture tree holding `route` under the dispatch dir and, when given,
    `table` as retired.rs; returns what mk_retired reads from it."""
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        (root / DISPATCH).mkdir(parents=True)
        (root / DISPATCH / "route.rs").write_text(f"fn route(nr: u64) -> i64 {{\n{route}}}\n")
        if table is not None:
            (root / RETIRED).write_text(table)
        return mk_retired(root, DISPATCH)


TREES = [
    ("the gate in route", GATE, TABLE, {"MXRT", "MXRU"}),
    ("the gate taken out of route", "", TABLE, set()),
    ("no retired.rs", GATE, None, set()),
]


# (case, the arms in route, every name they should count)
ARMS = [
    ("one name per arm", "        SYS_A => 0,\n        SYS_B => 1,\n", {"SYS_A", "SYS_B"}),
    ("an or-pattern", "        SYS_A | SYS_B => 0,\n", {"SYS_A", "SYS_B"}),
    ("an or-pattern over lines", "        SYS_A\n        | SYS_B => 0,\n", {"SYS_A", "SYS_B"}),
]


def arms(route, arm_re):
    """Every name mk_arms counts in a fixture tree whose route holds `route`."""
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        (root / DISPATCH).mkdir(parents=True)
        (root / DISPATCH / "route.rs").write_text(f"match nr {{\n{route}}}\n")
        return mk_arms(root, DISPATCH, arm_re)


def self_test(check_published, arm_re):
    failed = []
    for case, edits, want in CASES:
        got = check_published(TAG, tag4(TAG), kernel(**edits), {TAG})
        if len(got) != len(want) or not all(w in g for w, g in zip(want, got)):
            failed.append(f"{case}: wanted {want}, got {got}")
    for case, route, table, want in TREES:
        got = tree(route, table)
        if got != want:
            failed.append(f"{case}: wanted {sorted(want)}, got {sorted(got)}")
    for case, route, want in ARMS:
        got = arms(route, arm_re)
        if got != want:
            failed.append(f"{case}: wanted {sorted(want)}, got {sorted(got)}")
    for line in failed:
        print(f"syscall-abi self-test: {line}")
    if failed:
        return 1
    print(f"syscall-abi: self-test passed, {len(CASES) + len(TREES)} retired cases, {len(ARMS)} arm shapes")
    return 0

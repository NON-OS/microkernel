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
"""check_gate_reentry.py's self-test. The decoy is the reclaim that froze
the desktop: a GATE holder calling a function that takes GATE, directly and
through one more call. Taken after the guard's block, after drop(guard), or
named only in a comment or a string, it passes."""

import tempfile
from pathlib import Path

TAKER = "pub(super) fn reclaim_dma() {\n    let _gate = lock_responsive(&GATE);\n}\n"
HOP = "fn tidy() {\n    reclaim_dma();\n}\n"

CASES = [
    ("pub fn reclaim() {\n    let _gate = lock_responsive(&GATE);\n    reclaim_dma();\n}\n", 1),
    ("pub fn reclaim() {\n    let _gate = lock_responsive(&GATE);\n    tidy();\n}\n", 1),
    ("pub fn reclaim() {\n    let g = crate::smp::lock_responsive(&super::gate::GATE);\n"
     "    let _ = GATE.lock();\n    drop(g);\n}\n", 1),
    ("pub fn reclaim() {\n    reclaim_orphans();\n    reclaim_dma();\n}\n"
     "fn reclaim_orphans() {\n    let _gate = lock_responsive(&GATE);\n}\n", 0),
    ("pub fn reclaim() {\n    {\n        let _gate = lock_responsive(&GATE);\n    }\n    tidy();\n}\n", 0),
    ("pub fn reclaim() {\n    let g = lock_responsive(&GATE);\n    drop(g);\n    reclaim_dma();\n}\n", 0),
    ("pub fn reclaim() {\n    let _gate = lock_responsive(&GATE);\n"
     "    // reclaim_dma();\n    say(\"tidy()\");\n}\n", 0),
]


def self_test(check, base: str) -> int:
    for i, (reclaim, want) in enumerate(CASES):
        with tempfile.TemporaryDirectory() as d:
            pin = Path(d) / base / "pin"
            pin.mkdir(parents=True)
            (pin / "dma_held.rs").write_text(TAKER + HOP)
            (pin / "orphans.rs").write_text(reclaim)
            if check(Path(d)) != want:
                print(f"gate-reentry: self-test failed on case {i}: want exit {want}")
                return 1
    print("gate-reentry: self-test passed: a holder calling a taker, at one hop or two, is caught")
    return 0

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
"""The enrolment tool compiles in none of the trust files it writes.

nonos-stark-enroll writes the policy roots under nonos-data/trust/. A crate
it builds that include_bytes!s one is rebuilt whenever an enrolment rewrites
it, the tool's binary comes out newer than every stamp made with it, and the
next sub-make enrols again: the kernel was re-enrolled under a new root
after the loader had baked the old one, and the loader refused it at boot
("kernel self-attestation FAILED"). So no crate the tool builds by path,
with the features the tool builds it with, may compile such an include in.
"""

import argparse
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from enrol_inputs_features import enabled  # noqa: E402
from enrol_inputs_fixture import CASES, write  # noqa: E402
from enrol_inputs_scan import trust_includes  # noqa: E402

TOOL = "nonos-stark-enroll"


def compiled_in(tool: Path) -> list:
    return [f for crate, on in enabled(tool).items() for f, kept in trust_includes(crate, on) if kept]


def self_test() -> int:
    bad = 0
    for what, edge, gate, features, expect in CASES:
        with tempfile.TemporaryDirectory() as d:
            got = bool(compiled_in(write(Path(d), edge, gate, features)))
        if got != expect:
            print(f"enrol-inputs self-test: {what}: compiled in {got}, expected {expect}")
            bad += 1
    print(f"enrol-inputs self-test: {len(CASES)} cases, {bad} wrong")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--root", default=".")
    a = ap.parse_args()
    if a.self_test:
        return self_test()
    tool = Path(a.root) / TOOL
    found = compiled_in(tool)
    for f in found:
        print(f"enrol-inputs: {TOOL} compiles in a trust file: {f}")
    print(f"enrol-inputs: {len(enabled(tool))} crates built, {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())

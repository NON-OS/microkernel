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
"""Every capsule the kernel embeds refuses to compile with an empty file.

A capsule's feature on, an include_bytes! that picked up an empty ELF, cert,
manifest or trailer built a kernel whose spawn of it read as a cert decode
failure, or now, at the spawn gate, as "not built" (B-02). Each module that
embeds a capsule's ELF carries `const _: () = assert!(...)` naming every
const it includes, so such a build stops at compile time. The tool capsules
embed through one macro, which carries its own assert.
"""

import argparse
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from embed_assert_fixture import CASES, write  # noqa: E402
from embed_assert_scan import problems  # noqa: E402

def self_test() -> int:
    bad = 0
    for what, files, expect in CASES:
        with tempfile.TemporaryDirectory() as d:
            got = len(problems(write(Path(d), files)))
        if got != expect:
            print(f"embed-asserts self-test: {what}: {got} problems, expected {expect}")
            bad += 1
    print(f"embed-asserts self-test: {len(CASES)} cases, {bad} wrong")
    return 1 if bad else 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n")[0])
    ap.add_argument("--self-test", action="store_true")
    ap.add_argument("--src", default="src")
    a = ap.parse_args()
    if a.self_test:
        return self_test()
    found = problems(Path(a.src))
    for p in found:
        print(f"embed-asserts: {p}")
    print(f"embed-asserts: {len(found)} problems")
    return 1 if found else 0


if __name__ == "__main__":
    sys.exit(main())

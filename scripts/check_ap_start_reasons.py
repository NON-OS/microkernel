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
"""Every way an AP's start can fail says why.

`ap_unit::start` returns Err when the AP's stack or boot context cannot be
made, before INIT is sent; folding that into `unwrap_or(false)` hid the reason
(ap_start.rs:59 until this check). So:

  1. Only `src/smp/init/ap_start/one.rs` calls `ap_unit::start`.
  2. It matches the result with an `Err(why)` arm that prints
     "not started: " and the reason, and an `Ok` arm.
  3. Nothing under src/smp/init discards that result: no `unwrap_or`, `.ok()`,
     `let _` or `.unwrap_or_default()` on it.

    scripts/check_ap_start_reasons.py [--self-test]
"""

import re
import sys
from pathlib import Path

ONE = "src/smp/init/ap_start/one.rs"
CALL = re.compile(r"ap_unit::start\(")
DISCARD = re.compile(r"ap_unit::start\([^;]*?\)\s*\.\s*(unwrap_or|ok\(|unwrap_or_default|unwrap\(|expect\()"
                     r"|let\s+_\s*=\s*(?:super::)*ap_unit::start\(", re.S)
REASON = re.compile(r'Err\(\s*why\s*\)\s*=>\s*\{[^}]*"\s*not started: "\s*\)\s*\.str\(\s*why\.as_bytes\(\)\s*\)', re.S)


def strip(text):
    return re.sub(r"//[^\n]*", "", text)


def check(files):
    failed = []
    for path, text in files:
        body = strip(text)
        if DISCARD.search(body):
            failed.append(f"{path}: the result of ap_unit::start is discarded")
        if CALL.search(body) and path != ONE and not path.endswith("ap_unit.rs"):
            failed.append(f"{path}: calls ap_unit::start; only {ONE} may")
    one = dict(files).get(ONE)
    if one is None:
        failed.append(f"{ONE} is missing")
    else:
        body = strip(one)
        if not REASON.search(body) or "Ok(" not in body:
            failed.append(f"{ONE}: the Err arm does not print 'not started: ' and the reason")
    return failed


def tree(root):
    for p in sorted((root / "src/smp/init").rglob("*.rs")):
        yield p.relative_to(root).as_posix(), p.read_text()


def self_test(files):
    files = list(files)
    def swap(path, old, new):
        return [(p, t.replace(old, new) if p == path else t) for p, t in files]
    one = dict(files)[ONE]
    cases = [
        (files, True),
        (swap(ONE, "match super::super::ap_unit::start(cpu_id, apic_id, boot) {",
              "match super::super::ap_unit::start(cpu_id, apic_id, boot).ok() {"), False),
        (swap(ONE, '.str(b" not started: ")', '.str(b" failed")'), False),
        (files + [("src/smp/init/ap_start.rs.extra",
                   "if super::ap_unit::start(cpu_id, apic_id, &boot).unwrap_or(false) {}")], False),
        (files + [("src/smp/init/other.rs", "let _ = super::ap_unit::start(1, 2, &b);")], False),
        ([f for f in files if f[0] != ONE], False),
    ]
    assert one
    wrong = [i for i, (fs, ok) in enumerate(cases) if (not check(fs)) != ok]
    print(f"ap-start-reasons: self-test {'failed on ' + str(wrong) if wrong else 'passed'}, "
          f"{sum(not ok for _, ok in cases)} mutants rejected")
    return 1 if wrong else 0


def main():
    files = list(tree(Path(".")))
    if "--self-test" in sys.argv[1:]:
        return self_test(files)
    failed = check(files)
    for line in failed:
        print(f"ap-start-reasons: {line}")
    if not failed:
        print("ap-start-reasons: every failed AP start says why")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())

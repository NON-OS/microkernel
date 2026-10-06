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

"""Pass only when the booted kernel checked the loader and admitted it: one
"[BOOT-ATTEST] bootloader ... enrolled, epoch N" line, and no refusal. With
--measured, only the measured line passes (a TPM was there to read)."""

import argparse
import re
import sys

MEASURED = re.compile(r"\[BOOT-ATTEST\] bootloader measured and enrolled, epoch (\d+)")
SELF = re.compile(r"\[BOOT-ATTEST\] bootloader self-reported, not measured: enrolled, epoch (\d+)")
BAD = re.compile(r"\[BOOT-ATTEST\] bootloader (refused, code \d+|not checked[^\n]*)")


def judge(text, measured):
    bad = [f"the kernel logged: {m.group(0)}" for m in BAD.finditer(text)]
    m, s = MEASURED.search(text), SELF.search(text)
    if measured and not m:
        bad.append("no measured admission (was swtpm attached?)")
    elif not (m or s):
        bad.append("no [BOOT-ATTEST] admission line")
    return (m or s), bad


def self_test():
    ok = "[BOOT-ATTEST] bootloader measured and enrolled, epoch 1\n"
    weak = "[BOOT-ATTEST] bootloader self-reported, not measured: enrolled, epoch 1\n"
    assert not judge(ok, True)[1] and not judge(weak, False)[1]
    assert judge(weak, True)[1] and judge("", False)[1]
    assert judge(ok + "[BOOT-ATTEST] bootloader refused, code 7\n", False)[1]
    assert judge("[BOOT-ATTEST] bootloader not checked: no boot-root record\n", False)[1]
    print("boot attest check: self-test PASS")


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("--log", help="the boot's serial log")
    ap.add_argument("--measured", action="store_true", help="require the measured line")
    ap.add_argument("--self-test", action="store_true", help="prove the check bites")
    a = ap.parse_args()
    if a.self_test:
        return self_test() or 0
    if not a.log:
        ap.error("--log is required")
    hit, bad = judge(open(a.log, errors="replace").read(), a.measured)
    if hit:
        print(f"  admitted: {hit.group(0)}")
    for b in bad:
        print(f"FAIL {b}")
    print("boot attest: PASS" if not bad else "boot attest: FAIL")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())

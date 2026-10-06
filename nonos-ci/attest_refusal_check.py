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

"""Read the refusal probe's serial log and pass only when the booted spawn gate
refused the four broken capsules, each with its own [ZK-ATTEST] FAIL line, and
admitted the honest one."""

import argparse
import re
import sys

CASES = ["flip", "extra_cap", "kernel_kind", "stale_epoch", "honest"]


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    ap.add_argument("--log", required=True, help="serial log of the probe boot")
    a = ap.parse_args()
    text = open(a.log, errors="replace").read()
    bad = []
    for case in CASES:
        if f"[ATTEST-PROBE] {case} as expected" not in text:
            bad.append(f"{case}: no 'as expected' line")
    fails = re.findall(r"\[ZK-ATTEST\] FAIL proof_io: ([^\n]*)", text)
    if len(fails) != 4:
        bad.append(f"{len(fails)} gate refusals logged, want 4")
    if not any("STARK proof refused code" in f for f in fails):
        bad.append("the flipped proof's refusal carries no nox_verify code")
    if "[ZK-ATTEST] ok proof_io" not in text:
        bad.append("the honest capsule was not admitted by the gate")
    for line in fails:
        print(f"  refused: {line}")
    for b in bad:
        print(f"FAIL {b}")
    print("attest refusal: PASS" if not bad else "attest refusal: FAIL")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main())

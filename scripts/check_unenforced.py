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
"""Controls that exist and do not run, printed by name, held to shrink.

Every serious defect here was one: a capability nothing consulted, a gate on
a path nothing took, a ceiling that logged and admitted. The list goes to
zero one fix at a time; scripts/baselines/unenforced.txt may only lose rows.
--list prints every remaining one, which the kernel build does each time.
"""

import sys
import tempfile
from pathlib import Path

import gate
from unenforced_scan import unenforced

BASELINE = Path("scripts/baselines/unenforced.txt")
DECOYS = {
    "src/security/decoy.rs": "pub fn verify_decoy() -> bool { true }\n"
    "pub const DECOY_LIMIT: u32 = 4;\n"
    'fn f() { log("not enforced, would refuse"); }\n',
    "src/user.rs": "use crate::security::decoy::verify_decoy;\n",
}
WANT = ["src/security/decoy.rs:1 verify_decoy", "src/security/decoy.rs:2 DECOY_LIMIT",
        "src/security/decoy.rs:3 logs"]


def self_test():
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        for rel, text in DECOYS.items():
            (root / rel).parent.mkdir(parents=True, exist_ok=True)
            (root / rel).write_text(text)
        found = unenforced(root)
        if found != WANT:
            print(f"unenforced: self-test failed, found {found}")
            return 1
    print("unenforced: self-test passed, each of the three shapes was reported")
    return 0


def main():
    ap = gate.parser(__doc__)
    ap.add_argument("--list", action="store_true", help="print every remaining control by name")
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    if args.list:
        found = unenforced(args.root)
        for s in found:
            print(f"[unenforced] {s}")
        print(f"[unenforced] {len(found)} controls exist and do not run")
        return 0
    return gate.run("unenforced", "a new control that does not run at", unenforced, BASELINE, args)


if __name__ == "__main__":
    sys.exit(main())

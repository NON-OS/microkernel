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
"""Check the published capability table in the docs against the kernel.

The docs mirror the kernel's table, and had drifted (twenty-two variants
against twenty-eight, citing a path long gone). A reader who grants what the
page says gets what the kernel says, silently. A check that cannot find its
pages fails: this one skipped for years, reading a page never in the tree.
"""

import argparse
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_caps_abi import read_kernel  # noqa: E402
from docs_caps_table import check  # noqa: E402

# The bit table, and the page that says how many bits there are.
PAGES = (Path("docs/kernel/capabilities-what-each-bit.md"),
         Path("docs/kernel/capabilities-one-definition.md"))


def self_test() -> int:
    kernel = {"a": ("CoreExec", 1 << 0), "b": ("IO", 1 << 1)}
    rows = "| 0 | `CoreExec` | x |\n| 1 | `IO` | y |\n"
    with tempfile.TemporaryDirectory() as d:
        root = Path(d)
        got = [check(root, kernel, PAGES)]
        (root / PAGES[0]).parent.mkdir(parents=True)
        (root / PAGES[0]).write_text(rows)
        (root / PAGES[1]).write_text("There are 2, on bits 0 to 1.\n")
        got.append(check(root, kernel, PAGES))
        (root / PAGES[0]).write_text(rows.replace("| 1 | `IO`", "| 2 | `IO`"))
        got.append(check(root, kernel, PAGES))
    if got != [2, 0, 1]:
        print(f"docs-caps: self-test failed, got {got} for missing, good, wrong")
        return 1
    print("docs-caps: self-test passed: a missing page fails, a wrong bit is caught")
    return 0


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    ap.add_argument("--self-test", action="store_true", help="run on fixtures")
    args = ap.parse_args()
    if args.self_test:
        return self_test()
    try:
        kernel = read_kernel(args.root)
    except FileNotFoundError as e:
        print(f"docs-caps: {e}", file=sys.stderr)
        return 2
    return check(args.root, kernel, PAGES)


if __name__ == "__main__":
    sys.exit(main())

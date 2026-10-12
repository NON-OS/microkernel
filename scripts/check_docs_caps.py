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

The docs are a mirror of the same table the kernel enum defines, and they had
drifted: the page said twenty-two variants against a kernel that enforced
twenty-eight, and cited `src/capabilities/types.rs`, a path that no longer
exists.

A stale table in documentation is worse than none. A reader who grants what
the page says gets what the kernel says, and the difference is silent.

The docs live in their own repository, checked out here as a submodule, so
this skips rather than fails when it is not present.
"""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from check_caps_abi import read_kernel  # noqa: E402
from docs_caps_table import differences, table  # noqa: E402

PAGE = Path("docs/security/capabilities-and-tokens.md")
def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."), help="repository root")
    args = ap.parse_args()

    page = args.root / PAGE
    if not page.exists():
        print(f"docs-caps: {PAGE} not checked out, skipping")
        return 0
    try:
        kernel = read_kernel(args.root)
    except FileNotFoundError as e:
        print(f"docs-caps: {e}", file=sys.stderr)
        return 2

    text = page.read_text()
    documented = table(text)
    if not documented:
        print("docs-caps: found no bit table on the page", file=sys.stderr)
        return 2

    problems = differences(text, documented, kernel)

    if problems:
        print("docs-caps: the documented capability table is out of date",
              file=sys.stderr)
        for p in problems:
            print(p, file=sys.stderr)
        return 1
    print(f"docs-caps: {len(documented)} documented capabilities agree with the kernel")
    return 0


if __name__ == "__main__":
    sys.exit(main())

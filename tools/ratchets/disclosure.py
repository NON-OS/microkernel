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
"""The syscalls the Linux personality serves, and the ones x86_64 defines."""
"""Every served Linux call says what it discloses, in one file.

abi/disclosure.txt holds one line per served call: name | discloses | why that
is acceptable. A call served without a line fails, and so does a line for a
call that is not served, so the file cannot drift from the table.
"""

import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from linux_calls import LINUX, defined, served  # noqa: E402


def lines(root):
    out = {}
    for n, raw in enumerate((root / LINUX / "abi/disclosure.txt").read_text().splitlines(), 1):
        if not raw or raw.startswith("#"):
            continue
        parts = [p.strip() for p in raw.split("|")]
        if len(parts) != 3 or not all(parts):
            sys.exit(f"disclosure.txt:{n}: want name | discloses | why")
        out[parts[0]] = n
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--root", type=Path, default=Path("."))
    a = ap.parse_args()
    table = defined(a.root)
    have = {table[n] for n in served(a.root)}
    said = lines(a.root)
    missing = sorted(have - said.keys())
    extra = sorted(said.keys() - have)
    for name in missing:
        print(f"[disclosure] served without a line: {name}")
    for name in extra:
        print(f"[disclosure] a line for a call not served: {name}")
    print(f"[disclosure] {len(said)} lines, {len(have)} served")
    return 1 if missing or extra else 0


if __name__ == "__main__":
    sys.exit(main())

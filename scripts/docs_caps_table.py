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
"""check_docs_caps.py's reading of the pages: the bit table, and the count they state."""

import re
import sys

ROW = re.compile(r"^\| (\d+) \| `([A-Z][A-Za-z]+)` \|", re.M)


def table(text: str):
    """The bit table's rows, `| bit | `Name` | ... |`, as name -> value."""
    return {name: int(bit) for bit, name in ROW.findall(text)}


def differences(text: str, documented: dict, kernel) -> list:
    """Each way the page's table and count disagree with the kernel."""
    # The kernel gives each bit's mask; the page, its number.
    expected = {name: mask.bit_length() - 1 for name, mask in kernel.values()}
    problems = []
    for name, bit in sorted(documented.items(), key=lambda kv: kv[1]):
        if name not in expected:
            problems.append(f"  the page lists {name}, which the kernel does not define")
        elif expected[name] != bit:
            problems.append(f"  {name} DISAGREES: kernel {expected[name]}, page {bit}")
    for name, bit in sorted(expected.items(), key=lambda kv: kv[1]):
        if name not in documented:
            problems.append(f"  {name} ({bit}) is enforced and the page omits it")

    if f"There are {len(expected)}," not in text:
        problems.append(f"  the page does not say there are {len(expected)}")
    for dead in re.findall(r"`src/capabilities/types\.rs[^`]*`", text):
        problems.append(f"  cites {dead}, which no longer exists")
    return problems


def check(root, kernel, pages) -> int:
    """0 when the pages agree with the kernel, 1 when not, 2 when unreadable."""
    missing = [p for p in pages if not (root / p).exists()]
    if missing:
        print(f"docs-caps: {', '.join(map(str, missing))} not found", file=sys.stderr)
        return 2
    text = "\n".join((root / p).read_text() for p in pages)
    documented = table(text)
    if not documented:
        print("docs-caps: found no bit table on the pages", file=sys.stderr)
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

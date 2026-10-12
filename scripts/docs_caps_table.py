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
"""check_docs_caps.py's reading of the page: the bit table, and the count in words."""

import re

ENTRY = re.compile(r"\b([A-Z][A-Za-z]+)\s+(\d+)\b")
WORDS = {22: "twenty-two", 23: "twenty-three", 27: "twenty-seven",
         28: "twenty-eight", 29: "twenty-nine", 30: "thirty",
         31: "thirty-one", 32: "thirty-two", 33: "thirty-three",
         34: "thirty-four", 35: "thirty-five", 36: "thirty-six",
         37: "thirty-seven"}


def table(text: str):
    """The fenced block holding the bit table, as name -> value."""
    block = re.search(r"```\n(.*?CoreExec.*?)```", text, re.S)
    if not block:
        return {}
    return {n: int(v) for n, v in ENTRY.findall(block.group(1))}


def differences(text: str, documented: dict, kernel) -> list:
    """Each way the page's table and count disagree with the kernel."""
    expected = {name: bit for name, bit in kernel.values()}
    problems = []
    for name, bit in sorted(documented.items(), key=lambda kv: kv[1]):
        if name not in expected:
            problems.append(f"  the page lists {name}, which the kernel does not define")
        elif expected[name] != bit:
            problems.append(f"  {name} DISAGREES: kernel {expected[name]}, page {bit}")
    for name, bit in sorted(expected.items(), key=lambda kv: kv[1]):
        if name not in documented:
            problems.append(f"  {name} ({bit}) is enforced and the page omits it")

    word = WORDS.get(len(expected))
    if word and word not in text:
        problems.append(f"  the page does not say the set has {word} variants")
    for dead in re.findall(r"`src/capabilities/types\.rs[^`]*`", text):
        problems.append(f"  cites {dead}, which no longer exists")
    return problems

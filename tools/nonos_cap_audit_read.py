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
"""nonos-cap-audit's readers: the source, the declared capabilities, code lines."""

import re
from pathlib import Path


def read(root: Path, rel: str) -> str:
    return (root / rel).read_text(errors="replace")


def declared(root: Path) -> set:
    """The one list, as `Name = 1 << n,` inside the table invocation."""
    src = read(root, "src/capabilities/types/defs.rs")
    return set(re.findall(r"^\s{4}(\w+) = 1 << \d+,$", src, re.M))


def code_lines(path: Path):
    """Source lines with the comments dropped.

    Comments describe these checks using stand-in names, so a scan that
    reads them finds capabilities that were never meant to exist.
    """
    for line in path.read_text(errors="replace").splitlines():
        if line.lstrip().startswith(("//", "///", "*")):
            continue
        yield line


def called_anywhere(root: Path, fn: str) -> bool:
    """Whether anything outside its own definition calls `fn`."""
    pat = re.compile(r"\b" + re.escape(fn) + r"\s*\(")
    definition = re.compile(r"\bfn\s+" + re.escape(fn) + r"\b")
    for path in (root / "src").rglob("*.rs"):
        for line in code_lines(path):
            if definition.search(line):
                continue
            if pat.search(line):
                return True
    return False

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

import re
from pathlib import Path

LINUX = Path("userland/capsule_linux")
CONST = re.compile(r"pub const ([A-Z0-9_]+): u64 = (\d+);")
USE = re.compile(r"\bn[pr]::([A-Z0-9_]+)")


def served(root):
    numbers = {}
    for p in (root / LINUX / "src/linux/abi").glob("nr*.rs"):
        numbers.update({k: int(v) for k, v in CONST.findall(p.read_text())})
    used = set()
    for p in (root / LINUX / "src/linux/serve").glob("*.rs"):
        used |= set(USE.findall(p.read_text()))
    return {numbers[u] for u in used if u in numbers}


def defined(root):
    out = {}
    for line in (root / LINUX / "abi/x86_64-syscalls.txt").read_text().splitlines():
        if line and not line.startswith("#"):
            n, name = line.split()
            out[int(n)] = name
    return out

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
"""check_mirror_caps: the kernel's capability bits, a spawn mask's value,
a Capsule.mk field."""

import re

DEFS = "src/capabilities/types/defs.rs"


def capability_bits(root):
    bits = {}
    for m in re.finditer(r"^\s*(\w+)\s*=\s*([^,]+),", (root / DEFS).read_text(), re.M):
        expr = m.group(2).strip()
        bits[m.group(1)] = (1 << int(expr.split("<<")[1])) if "<<" in expr else int(expr, 0)
    return bits


def value(expr, bits, consts):
    v = 0
    for name in re.findall(r"Capability::(\w+)\.bit\(\)", expr):
        v |= bits[name]
    if "serial_debug_cap()" in expr:
        v |= bits["Debug"]
    for name in re.findall(r"\b([A-Z][A-Z0-9_]+)\b", expr):
        v |= consts.get(name, 0)
    for lit in re.findall(r"\b0x[0-9a-fA-F]+\b", expr):
        v |= int(lit, 16)
    return v


def manifest(text, key):
    m = re.search(rf"^{key}\s*:=\s*(\S+)", text, re.M)
    return m.group(1) if m else None

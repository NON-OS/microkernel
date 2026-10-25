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
"""nonos-cap-audit: a constant made from a capability's bit is no check.

`pub const CAP_ENTROPY: u64 = Capability::Entropy.bit();` names the bit and
asks nobody about it; the capability is consulted only where the constant
itself is read. So a definition is set aside as an alias, and counts once
some other line uses the alias's name. A `use` that re-exports it reads
nothing.
"""

import re
from pathlib import Path

from nonos_cap_audit_read import code_lines

DEFINES = re.compile(r"\b(?:const|static)\s+(\w+)\s*:[^=]*=.*Capability::(\w+)\.bit\(\)")
USES = re.compile(r"\b(?:pub(?:\([^)]*\))?\s+)?use\s+[^;]*;", re.S)


def alias(line: str):
    """(constant, capability) when `line` only defines a constant from a bit."""
    found = DEFINES.search(line)
    return (found.group(1), found.group(2)) if found else None


def used_elsewhere(root: Path, name: str) -> bool:
    """Whether any line other than `name`'s own definition reads it."""
    pat = re.compile(r"\b" + re.escape(name) + r"\b")
    for path in (root / "src").rglob("*.rs"):
        lines = [l for l in code_lines(path) if (alias(l) or ("",))[0] != name]
        if pat.search(USES.sub("", "\n".join(lines))):
            return True
    return False

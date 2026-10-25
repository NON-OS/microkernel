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
"""Which licences a catalogue listing may carry: open-source licences that
permit redistribution, as SPDX names, the way Alpine's index writes them.

Every licence a package names must be on the list, whichever way the
expression joins them: a package offered under "X OR Y" is still refused
when Y is not known here, because the catalogue does not choose for the
person which of the two they took it under."""

import re

ALLOWED = frozenset((
    "0BSD", "AGPL-3.0-only", "AGPL-3.0-or-later", "Apache-2.0", "Artistic-2.0",
    "BSD-2-Clause", "BSD-3-Clause", "BSL-1.0", "CC0-1.0", "GPL-2.0-only",
    "GPL-2.0-or-later", "GPL-3.0-only", "GPL-3.0-or-later", "ISC",
    "LGPL-2.0-or-later", "LGPL-2.1-only", "LGPL-2.1-or-later", "LGPL-3.0-only",
    "LGPL-3.0-or-later", "MIT", "MPL-2.0", "PSF-2.0", "Unlicense", "Vim", "X11",
    "Zlib", "curl", "blessing",
))
# Exceptions only ever widen what a licence allows.
EXCEPTIONS = frozenset(("OpenSSL-Exception", "GCC-exception-2.0", "Classpath-exception-2.0"))
JOINS = frozenset(("AND", "OR", "WITH"))


def names(expr: str) -> list:
    """The licence and exception names in an expression, joins dropped."""
    return [w for w in re.split(r"[\s()]+", expr) if w and w not in JOINS]


def redistributable(expr: str) -> bool:
    """True when the expression names at least one allowed licence and every
    name in it is an allowed licence or exception."""
    found = names(expr)
    return any(n in ALLOWED for n in found) and all(n in ALLOWED or n in EXCEPTIONS for n in found)

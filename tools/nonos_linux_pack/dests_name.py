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
"""A name as a destination list holds it: lowercase LDH labels of 1 to 63
bytes, no hyphen at either end, single dots, 253 bytes at most, no trailing
dot, and a last label that is not all digits, which is an address."""

import re

LABEL = re.compile(r"[a-z0-9]([a-z0-9-]{0,61}[a-z0-9])?")


def name_ok(name: str) -> bool:
    labels = name.split(".")
    return (0 < len(name) <= 253 and all(LABEL.fullmatch(l) for l in labels)
            and not labels[-1].isdigit())

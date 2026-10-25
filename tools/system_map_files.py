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
"""nonos_system_map's readers: a Capsule.mk field, an ELF machine, a size."""

import os
import struct

def field_of(path, key):
    """First value of a `KEY := value` line in a Capsule.mk."""
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as handle:
            for line in handle:
                if line.startswith(key):
                    parts = line.split(":=", 1)
                    if len(parts) == 2:
                        return parts[1].strip()
    except OSError:
        pass
    return ""


def machine_of(path):
    try:
        with open(path, "rb") as handle:
            head = handle.read(20)
    except OSError:
        return None
    if len(head) < 20 or head[:4] != b"\x7fELF":
        return None
    return struct.unpack_from("<H", head, 18)[0]


def size_of(path):
    try:
        return os.path.getsize(path)
    except OSError:
        return 0

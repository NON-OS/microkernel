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
"""The header fields the walk reads, as install/tar_field.rs and
tar_path.rs read them."""


def octal(field: bytes):
    value = 0
    for b in field:
        if 0x30 <= b <= 0x37:
            value = value * 8 + b - 0x30
        elif b in (0x20, 0):
            break
        else:
            return None
    return value


def cstr(raw: bytes) -> bytes:
    cut = raw.find(b"\0")
    return raw if cut < 0 else raw[:cut]


def name_of(head: bytes) -> bytes:
    name, prefix = cstr(head[:100]), cstr(head[345:500])
    return prefix + b"/" + name if head[257:262] == b"ustar" and prefix else name


def member(name: bytes) -> bytes:
    while name.startswith(b"./"):
        name = name[2:]
    while len(name) > 1 and name.endswith(b"/"):
        name = name[:-1]
    return name

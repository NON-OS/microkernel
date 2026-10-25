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

"""The libraries a program needs, and where it says to look."""

import struct

from .elf import DT_NEEDED, DT_RPATH, DT_RUNPATH, DT_STRSZ, DT_STRTAB, PT_DYNAMIC, vaddr_to_off


def dynamic(data: bytes, headers):
    """(needed names, runpath entries) from the dynamic section."""
    seg = next((p for p in headers if p[0] == PT_DYNAMIC), None)
    if seg is None:
        return [], []
    _, off, _, size = seg
    tags = []
    at = off
    while at + 16 <= off + size:
        tag, val = struct.unpack_from("<qQ", data, at)
        at += 16
        if tag == 0:
            break
        tags.append((tag, val))
    strtab = next((v for t, v in tags if t == DT_STRTAB), None)
    strsz = next((v for t, v in tags if t == DT_STRSZ), 0)
    if strtab is None:
        return [], []
    base = vaddr_to_off(headers, strtab)
    if base is None:
        return [], []
    table = data[base:base + strsz]

    def name(index: int) -> str:
        end = table.find(b"\0", index)
        return table[index:end if end >= 0 else None].decode(errors="replace")

    needed = [name(v) for t, v in tags if t == DT_NEEDED]
    paths = []
    for t, v in tags:
        if t in (DT_RUNPATH, DT_RPATH):
            paths.extend(p for p in name(v).split(":") if p)
    return needed, paths

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

"""The ELF program headers, and the interpreter a program names."""

import struct

PT_INTERP = 3
PT_DYNAMIC = 2
DT_NEEDED = 1
DT_RUNPATH = 29
DT_RPATH = 15
DT_STRTAB = 5
DT_STRSZ = 10

DEFAULT_LIBDIRS = ("/lib", "/usr/lib", "/lib64", "/usr/lib64")


class NotElf(Exception):
    pass


def phdrs(data: bytes):
    """(type, offset, vaddr, filesz) for each program header."""
    if len(data) < 64 or data[:4] != b"\x7fELF":
        raise NotElf("not an ELF")
    if data[4] != 2 or data[5] != 1:
        raise NotElf("not 64-bit little-endian")
    e_phoff, = struct.unpack_from("<Q", data, 32)
    e_phentsize, e_phnum = struct.unpack_from("<HH", data, 54)
    out = []
    for i in range(e_phnum):
        at = e_phoff + i * e_phentsize
        p_type, = struct.unpack_from("<I", data, at)
        p_offset, p_vaddr = struct.unpack_from("<QQ", data, at + 8)
        p_filesz, = struct.unpack_from("<Q", data, at + 32)
        out.append((p_type, p_offset, p_vaddr, p_filesz))
    return out


def vaddr_to_off(headers, vaddr: int):
    """File offset of a virtual address, using the load segment holding it."""
    for p_type, p_offset, p_vaddr, p_filesz in headers:
        if p_type == 1 and p_vaddr <= vaddr < p_vaddr + p_filesz:
            return p_offset + (vaddr - p_vaddr)
    return None


def interp(data: bytes, headers) -> str:
    for p_type, p_offset, _, p_filesz in headers:
        if p_type == PT_INTERP:
            raw = data[p_offset:p_offset + p_filesz]
            return raw.split(b"\0", 1)[0].decode()
    return ""

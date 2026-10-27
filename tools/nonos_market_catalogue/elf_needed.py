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
"""What an x86-64 ELF asks the loader for: its interpreter (PT_INTERP) and
the shared objects its dynamic section names (DT_NEEDED), read from the
program headers alone, as the loader reads them. Anything that is not a
little-endian ELF64 for x86-64, or points outside the file, is refused."""

import struct

PT_LOAD, PT_DYNAMIC, PT_INTERP = 1, 2, 3
DT_NULL, DT_NEEDED, DT_STRTAB = 0, 1, 5


def needed(elf: bytes) -> tuple:
    """(interpreter or "", [DT_NEEDED names in order]), or ValueError."""
    if elf[:4] != b"\x7fELF" or elf[4:6] != b"\x02\x01" or len(elf) < 64 or struct.unpack_from("<H", elf, 18)[0] != 62:
        raise ValueError("not a little-endian ELF64 for x86-64")
    phoff, = struct.unpack_from("<Q", elf, 32)
    size, count = struct.unpack_from("<HH", elf, 54)
    if size != 56 or phoff + size * count > len(elf):
        raise ValueError("its program headers are not in the file")
    heads = [struct.unpack_from("<IIQQQQQQ", elf, phoff + i * size) for i in range(count)]
    loads = [(h[3], h[2], h[5]) for h in heads if h[0] == PT_LOAD]  # vaddr, offset, filesz
    interp = next((cstr(elf, h[2]) for h in heads if h[0] == PT_INTERP), "")
    dyn = next((h for h in heads if h[0] == PT_DYNAMIC), None)
    if dyn is None:
        return interp, []
    tags, at = [], dyn[2]
    while at + 16 <= min(len(elf), dyn[2] + dyn[5]):
        tag, val = struct.unpack_from("<qQ", elf, at)
        if tag == DT_NULL:
            break
        tags.append((tag, val))
        at += 16
    strtab = next((v for t, v in tags if t == DT_STRTAB), None)
    if strtab is None:
        raise ValueError("its dynamic section has no string table")
    base = offset(loads, strtab)
    return interp, [cstr(elf, base + v) for t, v in tags if t == DT_NEEDED]


def offset(loads: list, vaddr: int) -> int:
    """The file offset of `vaddr`, through the loaded segment that holds it."""
    for start, off, filesz in loads:
        if start <= vaddr < start + filesz:
            return off + vaddr - start
    raise ValueError(f"address {vaddr:#x} is in no loaded segment")


def cstr(elf: bytes, at: int) -> str:
    end = elf.find(b"\0", at)
    if at >= len(elf) or end < 0:
        raise ValueError("a name runs past the file")
    return elf[at:end].decode("ascii")

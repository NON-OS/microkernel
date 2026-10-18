#!/usr/bin/env python3
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
"""The arm64 Image a loader boots, made from the kernel ELF.

The same bytes `objcopy -O binary` writes: every PT_LOAD's file bytes at its
load address less the lowest one, gaps zero filled, nothing after the last file
byte (BSS is the kernel's to clear). start.S puts the arm64 Image header first,
which is what makes a loader, QEMU's -kernel among them, treat it as an Image:
placed at a 2 MiB aligned RAM base plus text_offset with the device tree in x0
(Linux arm64 boot protocol).
"""

import struct

from check_kernel_elf import PT_LOAD, Elf

MAGIC_AT, MAGIC = 0x38, b"ARM\x64"


def image(elf: Elf) -> bytes:
    loads = [(paddr, offset, filesz) for t, _, offset, _, paddr, filesz, _ in elf.headers()
             if t == PT_LOAD and filesz]
    if not loads:
        raise ValueError("no PT_LOAD segment with file bytes")
    base = min(paddr for paddr, _, _ in loads)
    out = bytearray(max(paddr + filesz for paddr, _, filesz in loads) - base)
    for paddr, offset, filesz in loads:
        out[paddr - base:paddr - base + filesz] = elf.data[offset:offset + filesz]
    if out[MAGIC_AT:MAGIC_AT + 4] != MAGIC:
        raise ValueError("the first load segment does not start with the arm64 Image header")
    return bytes(out)


def text_offset(img: bytes) -> int:
    return struct.unpack_from("<Q", img, 8)[0]

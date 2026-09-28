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
"""The kernel ELF is a loadable image for the architecture it claims, or the lane fails.

A build lane that only checks the exit status of cargo proves that rustc and
lld ran. It does not prove the output is something a loader can start. This
checks the properties the loader and the first instruction depend on:

  - the ELF class, byte order, type and machine match the target architecture
  - the entry point is the `_start` symbol the linker script names in ENTRY()
  - the entry point lies inside a PT_LOAD segment that is executable
  - no PT_LOAD segment is both writable and executable
  - the signed manifest and signature sections are present and not empty

Reads the ELF directly, so it runs on a host with no cross binutils.
"""

import argparse
import struct
import sys
from pathlib import Path

ET_EXEC = 2
PT_LOAD = 1
PF_X = 1
PF_W = 2
SHT_SYMTAB = 2

MACHINES = {"x86_64": 62, "aarch64": 183, "riscv64": 243}
ENTRY_SYMBOL = "_start"
REQUIRED_SECTIONS = (".nonos.manifest", ".nonos.sig")


class Elf:
    def __init__(self, data):
        if data[:4] != b"\x7fELF":
            raise ValueError("not an ELF file")
        if data[4] != 2 or data[5] != 1:
            raise ValueError("not a little-endian ELF64 file")
        self.data = data
        (self.e_type, self.e_machine, _, self.e_entry, self.e_phoff, self.e_shoff, _,
         _, self.e_phentsize, self.e_phnum, self.e_shentsize, self.e_shnum,
         self.e_shstrndx) = struct.unpack_from("<HHIQQQIHHHHHH", data, 16)

    def segments(self):
        for i in range(self.e_phnum):
            p_type, p_flags, _, p_vaddr, _, _, p_memsz, _ = struct.unpack_from(
                "<IIQQQQQQ", self.data, self.e_phoff + i * self.e_phentsize)
            yield p_type, p_flags, p_vaddr, p_memsz

    def read(self, vaddr, size):
        """The file bytes a loader places at `vaddr`, or None if no PT_LOAD holds them."""
        for i in range(self.e_phnum):
            p_type, _, p_offset, p_vaddr, _, p_filesz, _, _ = struct.unpack_from(
                "<IIQQQQQQ", self.data, self.e_phoff + i * self.e_phentsize)
            if p_type == PT_LOAD and p_vaddr <= vaddr and vaddr + size <= p_vaddr + p_filesz:
                at = p_offset + vaddr - p_vaddr
                return self.data[at:at + size]
        return None

    def sections(self):
        heads = []
        for i in range(self.e_shnum):
            heads.append(struct.unpack_from(
                "<IIQQQQIIQQ", self.data, self.e_shoff + i * self.e_shentsize))
        names = heads[self.e_shstrndx][4] if heads else 0
        for sh in heads:
            yield self.cstr(names + sh[0]), sh

    def symbol(self, wanted):
        by_index = [sh for _, sh in self.sections()]
        for sh in by_index:
            if sh[1] != SHT_SYMTAB:
                continue
            strtab = by_index[sh[6]][4]
            offset, size, entsize = sh[4], sh[5], sh[9]
            for at in range(offset, offset + size, entsize):
                st_name, _, _, _, st_value, _ = struct.unpack_from("<IBBHQQ", self.data, at)
                if self.cstr(strtab + st_name) == wanted:
                    return st_value
        return None

    def cstr(self, at):
        end = self.data.index(b"\0", at)
        return self.data[at:end].decode("ascii", "replace")


def check(elf, arch):
    """Every failed property, as one line each."""
    failed = []
    if elf.e_type != ET_EXEC:
        failed.append(f"e_type is {elf.e_type}, want {ET_EXEC} (a static executable)")
    if elf.e_machine != MACHINES[arch]:
        failed.append(f"e_machine is {elf.e_machine}, want {MACHINES[arch]} ({arch})")

    start = elf.symbol(ENTRY_SYMBOL)
    if start is None:
        failed.append(f"no `{ENTRY_SYMBOL}` symbol in the symbol table")
    elif start != elf.e_entry:
        failed.append(f"entry {elf.e_entry:#x} is not `{ENTRY_SYMBOL}` at {start:#x}")

    loads = [s for s in elf.segments() if s[0] == PT_LOAD]
    if not loads:
        failed.append("no PT_LOAD segment")
    holder = [s for s in loads if s[2] <= elf.e_entry < s[2] + s[3]]
    if not holder:
        failed.append(f"entry {elf.e_entry:#x} lies in no PT_LOAD segment")
    elif not holder[0][1] & PF_X:
        failed.append(f"entry {elf.e_entry:#x} lies in a segment without PF_X")
    for _, flags, vaddr, memsz in loads:
        if flags & PF_W and flags & PF_X:
            failed.append(f"segment at {vaddr:#x} (+{memsz:#x}) is writable and executable")

    sizes = {name: sh[5] for name, sh in elf.sections()}
    for name in REQUIRED_SECTIONS:
        if name not in sizes:
            failed.append(f"section {name} is missing")
        elif sizes[name] == 0:
            failed.append(f"section {name} is empty")
    return failed


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--elf", type=Path, required=True, help="the linked kernel ELF")
    ap.add_argument("--arch", choices=sorted(MACHINES), required=True)
    a = ap.parse_args()

    if not a.elf.is_file():
        print(f"kernel-elf: {a.elf} does not exist")
        return 2
    try:
        elf = Elf(a.elf.read_bytes())
    except (ValueError, struct.error) as e:
        print(f"kernel-elf: {a.elf}: {e}")
        return 1

    failed = check(elf, a.arch)
    if failed:
        print(f"kernel-elf: {a.elf} is not a loadable {a.arch} kernel:")
        for line in failed:
            print(f"  {line}")
        return 1

    loads = sum(1 for s in elf.segments() if s[0] == PT_LOAD)
    print(f"kernel-elf: {a.elf} is an executable for {a.arch}, entry `{ENTRY_SYMBOL}` at "
          f"{elf.e_entry:#x}, {loads} PT_LOAD segments, none writable and executable")
    return 0


if __name__ == "__main__":
    sys.exit(main())

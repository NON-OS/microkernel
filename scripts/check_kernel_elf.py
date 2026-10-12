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
  - aarch64, which relocates itself (src/arch/aarch64/boot/relocate), is a
    static PIE: no interpreter, no libraries, and every dynamic relocation an
    R_AARCH64_RELATIVE into a writable segment, the one kind its entry applies
  - the entry point is the `_start` symbol the linker script names in ENTRY()
  - the entry point lies inside a PT_LOAD segment that is executable
  - no PT_LOAD segment is both writable and executable
  - the signed manifest and signature sections are present and not empty, and
    no relocation rewrites a byte of either: they are signed as linked
  - aarch64 text signs return addresses and has BTI landing pads, in their
    hint-space forms only: the combined PAuth branches and loads are
    undefined on an ARMv8.0 core such as a Raspberry Pi 4's Cortex-A72

Reads the ELF directly, so it runs on a host with no cross binutils.
"""

import argparse
import struct
import sys
from pathlib import Path

ET_EXEC = 2
ET_DYN = 3
PT_LOAD = 1
PT_DYNAMIC = 2
PT_INTERP = 3
DT_NEEDED, DT_RELA, DT_RELASZ, DT_RELAENT, DT_REL, DT_JMPREL = 1, 7, 8, 9, 17, 23
R_AARCH64_RELATIVE = 1027
PF_X = 1
PF_W = 2
SHT_SYMTAB = 2

MACHINES = {"x86_64": 62, "aarch64": 183, "riscv64": 243}
TYPES = {"x86_64": ET_EXEC, "aarch64": ET_DYN, "riscv64": ET_EXEC}
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

    def headers(self):
        """(type, flags, offset, vaddr, paddr, filesz, memsz) of every program header."""
        for i in range(self.e_phnum):
            p_type, p_flags, p_offset, p_vaddr, p_paddr, p_filesz, p_memsz, _ = struct.unpack_from(
                "<IIQQQQQQ", self.data, self.e_phoff + i * self.e_phentsize)
            yield p_type, p_flags, p_offset, p_vaddr, p_paddr, p_filesz, p_memsz

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


def static_pie(elf):
    """What keeps an ET_DYN from being a static PIE the kernel's entry can relocate."""
    heads = list(elf.headers())
    if any(h[0] == PT_INTERP for h in heads):
        return ["a PT_INTERP asks for a dynamic linker"]
    dynamic = [h for h in heads if h[0] == PT_DYNAMIC]
    if not dynamic:
        return ["no PT_DYNAMIC, so its relocations cannot be found"]
    _, _, offset, _, _, filesz, _ = dynamic[0]
    tags = {}
    for at in range(offset, offset + filesz, 16):
        tag, value = struct.unpack_from("<qQ", elf.data, at)
        tags.setdefault(tag, value)
    failed = [f"dynamic tag {t} is a relocation or library the kernel does not apply"
              for t in (DT_NEEDED, DT_REL, DT_JMPREL) if t in tags]
    if tags.get(DT_RELASZ, 0) == 0:
        return failed
    if tags.get(DT_RELAENT) != 24:
        return failed + [f"DT_RELAENT is {tags.get(DT_RELAENT)}, want 24"]
    table = elf.read(tags.get(DT_RELA, 0), tags[DT_RELASZ])
    if table is None:
        return failed + ["the RELA table lies outside the loaded image"]
    writable = [(v, v + f) for t, fl, _, v, _, f, _ in heads if t == PT_LOAD and fl & PF_W]
    signed = [(name, sh[3], sh[3] + sh[5]) for name, sh in elf.sections() if name in REQUIRED_SECTIONS]
    for at in range(0, len(table), 24):
        place, info, _ = struct.unpack_from("<QQQ", table, at)
        if info & 0xFFFF_FFFF != R_AARCH64_RELATIVE:
            failed.append(f"relocation at {place:#x} has type {info & 0xFFFF_FFFF}, not RELATIVE")
        elif place % 8 or not any(lo <= place and place + 8 <= hi for lo, hi in writable):
            failed.append(f"relocation at {place:#x} is not 8 aligned file bytes of a writable segment")
        for name, lo, hi in signed:
            if place < hi and place + 8 > lo:
                failed.append(f"relocation at {place:#x} rewrites {name}, which is signed as linked")
        if len(failed) >= 8:
            return failed + ["(more not listed)"]
    return failed


PACIASP, AUTIASP = 0xD503233F, 0xD50323BF
BTI_PADS = (0xD503245F, 0xD503249F, 0xD50324DF)


def pauth_only(word):
    """RETAA/B, ERETAA/B, BRAA/B(Z), BLRAA/B(Z): an unconditional branch to a
    register with op3 0b00001x; LDRAA/B: 0b11111000 M S 1 imm9 W 1. None is a
    hint: each is undefined where FEAT_PAuth is not implemented."""
    branch = word & 0xFE000000 == 0xD6000000 and (word >> 10) & 0x3E == 0x2
    return branch or word & 0xFF200400 == 0xF8200400


def branch_protection(elf):
    """What keeps aarch64 text from being signed, padded and still ARMv8.0."""
    text = b"".join(elf.data[o:o + f] for t, fl, o, _, _, f, _ in elf.headers()
                    if t == PT_LOAD and fl & PF_X)
    words = struct.unpack_from(f"<{len(text) // 4}I", text)
    failed = []
    if PACIASP not in words or AUTIASP not in words:
        failed.append("no PACIASP/AUTIASP in text: not built with -Z branch-protection=pac-ret")
    if not any(w in BTI_PADS for w in words):
        failed.append("no BTI landing pad in text: not built with -Z branch-protection=bti")
    combined = sum(1 for w in words if pauth_only(w))
    if combined:
        failed.append(f"{combined} combined PAuth instructions, undefined on ARMv8.0: "
                      "is +pauth back in the target's features?")
    return failed


def check(elf, arch):
    """Every failed property, as one line each."""
    failed = []
    if elf.e_type != TYPES[arch]:
        failed.append(f"e_type is {elf.e_type}, want {TYPES[arch]}")
    elif elf.e_type == ET_DYN:
        failed += static_pie(elf)
    if elf.e_machine != MACHINES[arch]:
        failed.append(f"e_machine is {elf.e_machine}, want {MACHINES[arch]} ({arch})")
    if arch == "aarch64":
        failed += branch_protection(elf)

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
    kind = "a static PIE" if elf.e_type == ET_DYN else "an executable"
    print(f"kernel-elf: {a.elf} is {kind} for {a.arch}, entry `{ENTRY_SYMBOL}` at "
          f"{elf.e_entry:#x}, {loads} PT_LOAD segments, none writable and executable"
          + (", return addresses signed and branch targets padded" if a.arch == "aarch64" else ""))
    return 0


if __name__ == "__main__":
    sys.exit(main())

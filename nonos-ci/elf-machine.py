#!/usr/bin/env python3
# NONOS Operating System (AGPL-3.0-or-later)
"""Check one ELF is a little-endian ELF64 for the machine named.

Usage: elf-machine.py <e_machine> <path>. Exit 0 when it is; otherwise print
why on one line and exit 1. The header layout is the System V ABI's: the
class at byte 4, the data encoding at byte 5, e_machine at bytes 18 and 19.
A missing or empty file fails: make's rule once left an empty file where a
build for the wrong machine should have been.
"""

import struct
import sys


def why_not(machine: int, path: str) -> str | None:
    try:
        with open(path, "rb") as f:
            head = f.read(20)
    except OSError as e:
        return f"{path}: {e.strerror}"
    if len(head) < 20 or head[:4] != b"\x7fELF" or head[4] != 2 or head[5] != 1:
        return f"{path}: not a little-endian ELF64"
    got = struct.unpack_from("<H", head, 18)[0]
    return None if got == machine else f"{path}: e_machine {got}, want {machine}"


if __name__ == "__main__":
    if len(sys.argv) != 3 or not sys.argv[1].isdigit():
        sys.exit("usage: elf-machine.py <e_machine> <path>")
    why = why_not(int(sys.argv[1]), sys.argv[2])
    if why:
        print(why)
    sys.exit(1 if why else 0)

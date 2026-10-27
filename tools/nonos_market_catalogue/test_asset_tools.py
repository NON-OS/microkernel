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
"""What tools/nonos-source-asset holds an asset to: the one sha256 upstream's
signed file gives it, and the libraries its ELF asks for. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import struct
import unittest

from . import asset_sums, elf_needed

A, B = "ab" * 32, "cd" * 32


def elf(interp: bytes, names: list) -> bytes:
    """A little ELF64: one load of the whole file, its interpreter, a dynamic section."""
    strs = b"\0" + b"".join(n + b"\0" for n in names)
    body_at = 64 + 3 * 56
    interp_at, str_at = body_at, body_at + len(interp) + 1
    dyn = b"".join(struct.pack("<qQ", 1, 1 + sum(len(n) + 1 for n in names[:i])) for i in range(len(names)))
    dyn += struct.pack("<qQ", 5, 0x400000 + str_at) + struct.pack("<qQ", 0, 0)
    dyn_at = str_at + len(strs)
    total = dyn_at + len(dyn)
    head = b"\x7fELF\x02\x01\x01" + bytes(9) + struct.pack("<HHIQQQIHHHHHH", 2, 62, 1, 0, 64, 0, 0, 64, 56, 3, 0, 0, 0)
    ph = struct.pack("<IIQQQQQQ", 1, 5, 0, 0x400000, 0x400000, total, total, 4096)
    ph += struct.pack("<IIQQQQQQ", 3, 4, interp_at, 0, 0, len(interp) + 1, len(interp) + 1, 1)
    ph += struct.pack("<IIQQQQQQ", 2, 6, dyn_at, 0, 0, len(dyn), len(dyn), 8)
    return head + ph + interp + b"\0" + strs + dyn


class AssetTools(unittest.TestCase):
    def test_the_signed_file_gives_the_asset_one_sha256(self):
        text = f"{A}  v7.0.3/simplex-chat-ubuntu-22_04-x86_64\n{B}  v7.0.3/simplex-chat-ubuntu-24_04-x86_64\n"
        self.assertEqual(asset_sums.sum_of(text, "simplex-chat-ubuntu-22_04-x86_64"), A)
        self.assertEqual(asset_sums.sum_of(f"{A} *x\r\n", "x"), A)
        for bad in ("", f"{A}  v/x\n{B}  w/x\n", f"{A}  v/xy\n", f"{A}  v/ax\n", f"{A.upper()}  v/x\n", f"{A}\tv/x\n"):
            with self.assertRaises(ValueError, msg=bad):
                asset_sums.sum_of(bad, "x")
        good = f"[GNUPG:] VALIDSIG {'0' * 40} 2026-01-01 1 0 4 0 22 8 00 {asset_sums.FINGERPRINT}"
        self.assertEqual(asset_sums.signer("[GNUPG:] GOODSIG x\n" + good), asset_sums.FINGERPRINT)
        self.assertEqual(asset_sums.signer("[GNUPG:] BADSIG x y\n"), "")

    def test_an_elf_names_its_interpreter_and_libraries(self):
        got = elf_needed.needed(elf(b"/lib64/ld-linux-x86-64.so.2", [b"libgmp.so.10", b"libc.so.6"]))
        self.assertEqual(got, ("/lib64/ld-linux-x86-64.so.2", ["libgmp.so.10", "libc.so.6"]))
        for bad in (b"#!/bin/sh", elf(b"x", [])[:100], b"\x7fELF\x01\x01" + bytes(60)):
            with self.assertRaises(ValueError):
                elf_needed.needed(bad)

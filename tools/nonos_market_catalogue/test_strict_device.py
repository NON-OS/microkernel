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
"""Where the device reads what the reviews found Python reading otherwise:
a .PKGINFO only as pkginfo.rs reads it, a pax length past u64 as no
record, an octal size stopped at a space, the limits pinned to the Rust
that sets them, and a program past MAX_IMAGE refused. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import gzip
import unittest
from pathlib import Path

from . import alpine_auth, apk, gz
from .apk_fixture import ELF, tar
from .tar_fixture import rec
from .tarwalk import walk

RS = Path(__file__).resolve().parents[2] / "userland/capsule_linux/src/linux/install"


class Device(unittest.TestCase):
    def test_a_pkginfo_the_device_would_not_read_is_refused(self):
        hexd = "ab" * 32
        for body in (f"x = 1\rdatahash = {hexd}\n", f"datahash = {hexd}\x1f\n", f"datahash = {hexd.upper()}\n",
                     "datahash = \n", b"\xffdatahash = ".decode("latin-1") + hexd):
            with self.assertRaises(ValueError, msg=repr(body)):
                alpine_auth.datahash(tar([(".PKGINFO", body.encode("latin-1"))]))
        self.assertEqual(alpine_auth.datahash(tar([(".PKGINFO", f"datahash = {hexd} \r\n".encode())])),
                         bytes.fromhex(hexd))
        with self.assertRaises(ValueError):
            alpine_auth.datahash(tar([("PKGINFO", f"datahash = {hexd}\n".encode())]))

    def test_a_pax_length_past_u64_is_no_record_and_octal_stops_at_a_space(self):
        data = rec(b"p", b"x", b"9" * 20 + b" path=x\n") + rec(b"kept", b"0")
        self.assertEqual([e.name for e in walk(data)], [b"kept"])
        head = bytearray(rec(b"f", b"0", b"abc"))
        head[124:136] = b"0000000003 7"
        self.assertEqual([e.body for e in walk(bytes(head))], [b"abc"])

    def test_limits_are_the_rust_ones_and_a_program_past_max_image_is_refused(self):
        self.assertIn(f"MAX_INFLATED: usize = {gz.MAX_INFLATED >> 20} << 20;", (RS / "unpacked.rs").read_text())
        self.assertIn(f"MAX_IMAGE: u32 = {apk.MAX_IMAGE >> 20} << 20;", (RS / "market" / "bind.rs").read_text())
        big = ELF + bytes(apk.MAX_IMAGE)
        with self.assertRaises(ValueError):
            apk.program(tar([("usr/bin/x", big)]), "x")
        self.assertEqual(len(gzip.decompress(gzip.compress(b"x"))), 1)


if __name__ == "__main__":
    unittest.main()

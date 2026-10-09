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
"""`mirror` on small synthetic files served here: the layout, a resumed download,
bytes not the pin refused, the signed catalogue, and no download without a seed."""

import hashlib
import os
import tempfile
import unittest

from .ed25519 import public, verify
from .mirror import lay_out, mirror
from .rangeserve import Ranged, quietly, serve
from .wire_read import decode

A, B1, B2 = ("qwen2.5-0.5b-instruct-q4_k_m.gguf", "qwen2.5-14b-instruct-q4_k_m-00001-of-00002.gguf",
              "qwen2.5-14b-instruct-q4_k_m-00002-of-00002.gguf")
FILES = {A: os.urandom(1000), B1: os.urandom(3000), B2: b"tail"}
TABLE = {t: [(n, len(FILES[n]), hashlib.sha256(FILES[n]).hexdigest()) for n in names]
         for t, names in (("small", [A]), ("xxl", [B1, B2]))}


class Mirror(unittest.TestCase):
    def setUp(self):
        url, server = serve(dict(FILES))
        self.addCleanup(server.shutdown)
        self.out = self.enterContext(tempfile.TemporaryDirectory())
        self.source = lambda name: f"{url}/{name}"

    def test_layout_and_resume(self):
        os.makedirs(os.path.join(self.out, "xxl"))
        with open(os.path.join(self.out, "xxl", B1 + ".part"), "wb") as f:
            f.write(FILES[B1][:1200])
        quietly(lay_out, TABLE, ["small", "xxl"], self.out, self.source)
        for tier, files in TABLE.items():
            for name, _, _ in files:
                with open(os.path.join(self.out, tier, name), "rb") as f:
                    self.assertEqual(f.read(), FILES[name])
        self.assertIn(1200, Ranged.starts)

    def test_bytes_not_the_pin_are_refused(self):
        Ranged.files[A] = b"x" + FILES[A][1:]
        with self.assertRaises(SystemExit):
            quietly(lay_out, TABLE, ["small"], self.out, self.source)
        self.assertEqual(os.listdir(os.path.join(self.out, "small")), [])

    def test_catalogue_beside_the_files(self):
        seed, key = os.path.join(self.out, "seed"), os.path.join(self.out, "pub")
        with open(seed, "wb") as s, open(key, "wb") as k:
            s.write(bytes(32)), k.write(public(bytes(32)))
        base = "https://repo.nonos.test/qwen"
        quietly(mirror, TABLE, ["xxl"], self.out, base, 9, seed, key, self.source)
        with open(os.path.join(self.out, "catalogue.bin"), "rb") as f:
            serial, got, tiers, body, sig = decode(f.read())
        self.assertTrue(verify(public(bytes(32)), body, sig))
        self.assertEqual((serial, got, [t for t, _, _ in tiers]), (9, base, ["small", "xxl"]))
        self.assertEqual(tiers[1][2][0][3][0], f"{base}/xxl/{B1}")

    def test_no_seed_downloads_nothing(self):
        args = (TABLE, ["xxl"], self.out, "https://r.test", 9, "/no/seed", "/no/pub", self.source)
        self.assertRaises(SystemExit, quietly, mirror, *args)
        self.assertEqual(Ranged.starts, [])

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
"""The model catalogue against the pins, byte for byte; it is never built
otherwise. Run with the other host tests:

    python3 -m unittest discover -s tools/nonos_qwen_tier -t tools
"""

import unittest
from unittest import mock

from . import catalogue
from .catalogue import agree, body
from .pins import pins
from .wire_read import decode

BASE = "https://repo.nonos.test/qwen"


class Catalogue(unittest.TestCase):
    def setUp(self):
        self.table = pins()

    def test_every_pin_byte_for_byte(self):
        serial, base, tiers, _, _ = decode(body(self.table, BASE, 7) + bytes(64))
        self.assertEqual((serial, base, len(tiers)), (7, BASE, 17))
        self.assertTrue(agree(tiers, self.table))
        self.assertEqual(sum(len(f) for _, _, f in tiers), 28)
        for word, memory, files in tiers:
            self.assertGreater(memory, sum(length for _, length, _, _ in files))
            for name, _, _, urls in files:
                self.assertEqual(urls[0], f"{BASE}/{word}/{name}")
                self.assertTrue(urls[1].startswith("https://huggingface.co/Qwen/"))
                self.assertTrue(urls[1].endswith("/resolve/main/" + name))

    def test_upstream_only_without_a_base(self):
        _, base, tiers, _, _ = decode(body(self.table, "", 7) + bytes(64))
        self.assertEqual(base, "")
        self.assertTrue(all(len(urls) == 1 for _, _, f in tiers for *_, urls in f))

    def test_refused_when_it_and_the_pins_differ(self):
        real = catalogue.entries

        def off_by_one(table, base):
            tiers = real(table, base)
            name, length, sha, urls = tiers[3][2][1]
            tiers[3][2][1] = (name, length + 1, sha, urls)
            return tiers
        with mock.patch.object(catalogue, "entries", off_by_one):
            with self.assertRaises(SystemExit):
                body(self.table, BASE, 7)


if __name__ == "__main__":
    unittest.main()

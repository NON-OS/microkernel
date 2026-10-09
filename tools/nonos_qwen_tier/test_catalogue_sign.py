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
"""The catalogue's signature and the mirror bases the fetcher can reach.

    python3 -m unittest discover -s tools/nonos_qwen_tier -t tools
"""

import unittest

from .catalogue import body, signed
from .ed25519 import public, verify
from .layout import check_base
from .pins import pins
from .wire_read import decode

BASE = "https://repo.nonos.test/qwen"
SEED = bytes(range(32))


class Signed(unittest.TestCase):
    def setUp(self):
        self.table = pins()

    def test_signed_under_the_operator_key_only(self):
        raw = body(self.table, BASE, 7)
        blob = signed(raw, SEED, public(SEED))
        _, _, _, signed_body, sig = decode(blob)
        self.assertTrue(verify(public(SEED), signed_body, sig))
        self.assertFalse(verify(public(SEED), signed_body[:-1] + b"\1", sig))
        with self.assertRaises(SystemExit):
            signed(raw, SEED, public(bytes(32)))

    def test_bases_the_fetcher_cannot_reach_are_refused(self):
        self.assertEqual(check_base(BASE + "/"), BASE)
        for bad in ("http://repo.nonos.test", "https://repo.nonos.test:8443/q",
                    "https://repo.nonos.test/a b", "https://repo.nonos.test/q?x=1"):
            with self.assertRaises(SystemExit):
                check_base(bad)


if __name__ == "__main__":
    unittest.main()

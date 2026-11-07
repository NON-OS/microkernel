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

"""tools/nonos-dev-image pins the copy's SIMPLEX_CORE to the core's listing
key under the copy's market operator: under the checkout's operator it is
the key the checkout's serves.rs already holds, under a throwaway one the
key the copy's catalogue gives the core's guest policy, and nothing else in
the file moves. A file without exactly one constant and one listing to pair
is refused. Nothing here builds or copies anything."""

import os
import re
import unittest

from test_features import load

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(os.path.dirname(HERE))
SERVES = os.path.join(ROOT, "src/services/registry/serves.rs")
OPERATOR = os.path.join(ROOT, ".keys/marketplace_operator_ed25519.pub")
# A throwaway operator, and the key a copy signed under it gave the core's
# guest policy on a development guest ("[LINUX] policy bound: ... listing=").
THROWAWAY = bytes.fromhex("37e68c4475a011943b9fdb0a8f77a797f4a94c2623660eedd3c6f0dce27a893a")
ITS_KEY = "cde6dfb5c42457ce04afa9696cd9b5cdd2057bf7e7c23fbe1081279b3c4d75db"


def pinned(text):
    body = re.search(r"const SIMPLEX_CORE: \[u8; 32\] = \[\n(.*?)\n\];", text, re.S).group(1)
    return bytes(int(b, 16) for b in re.findall(r"0x([0-9a-f]{2})", body)).hex()


class Serves(unittest.TestCase):
    def setUp(self):
        self.t = load()
        with open(SERVES) as f:
            self.text = f.read()
        with open(OPERATOR, "rb") as f:
            self.operator = f.read()

    def test_the_checkouts_operator_gives_the_file_back_as_it_is(self):
        self.assertEqual(self.t.serves_text(self.text, self.operator), self.text)

    def test_a_throwaway_operator_gives_its_catalogues_key_and_moves_nothing_else(self):
        out = self.t.serves_text(self.text, THROWAWAY)
        self.assertEqual(pinned(out), ITS_KEY)
        self.assertNotEqual(pinned(self.text), ITS_KEY)
        strip = lambda s: re.sub(r"const SIMPLEX_CORE: \[u8; 32\] = \[\n.*?\n\];", "", s, flags=re.S)
        self.assertEqual(strip(out), strip(self.text))
        self.assertEqual(self.t.serves_text(out, THROWAWAY), out, "pinned once")

    def test_a_file_with_none_or_two_to_pair_or_a_short_key_is_refused(self):
        const = re.search(r"const SIMPLEX_CORE: \[u8; 32\] = \[\n.*?\n\];", self.text, re.S).group(0)
        listing = '("app.simplex", "linux.kali.nonos-simplex-core")'
        for text in (self.text.replace(const, ""), self.text + "\n" + const + "\n",
                     self.text.replace(listing, ""), self.text + listing):
            self.assertIsNone(self.t.serves_text(text, THROWAWAY))
        self.assertIsNone(self.t.serves_text(self.text, THROWAWAY[:31]))


if __name__ == "__main__":
    unittest.main()

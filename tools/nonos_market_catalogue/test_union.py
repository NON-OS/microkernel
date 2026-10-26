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

"""A development image's index: every release listing, then the four test
listings, all under the copy's key, and an id in both refused. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import copy
import json
import unittest
from pathlib import Path

from .union import union

ROOT = Path(__file__).resolve().parents[2]
RELEASE = json.loads((ROOT / "nonos-data/market/index.json").read_text())
TEST = json.loads((ROOT / "tools/devnet/market/index.json").read_text())
KEY = "ab" * 32
FOUR = ["devtest.free", "devtest.install", "devtest.metered", "devtest.escrow"]


class Union(unittest.TestCase):
    def joined(self):
        return union(copy.deepcopy(RELEASE), copy.deepcopy(TEST), KEY)

    def test_every_release_listing_then_the_four_test_ones(self):
        ids = [e["listing_id"] for e in self.joined()["entries"]]
        release = [e["listing_id"] for e in RELEASE["entries"]]
        self.assertGreater(len(release), 0)
        self.assertEqual(ids, release + FOUR)

    def test_the_release_listings_are_unchanged(self):
        self.assertEqual(self.joined()["entries"][: len(RELEASE["entries"])], RELEASE["entries"])

    def test_the_test_listings_are_the_copys_with_no_logo(self):
        for e in self.joined()["entries"][len(RELEASE["entries"]):]:
            self.assertEqual(e["publisher_pubkey"], KEY)
            self.assertEqual(e["logo_hash"], "")

    def test_an_id_in_both_is_refused(self):
        clash = copy.deepcopy(TEST)
        clash["entries"][0]["listing_id"] = RELEASE["entries"][0]["listing_id"]
        with self.assertRaises(SystemExit):
            union(copy.deepcopy(RELEASE), clash, KEY)

    def test_the_document_is_the_releases(self):
        got = self.joined()
        self.assertEqual({k: v for k, v in got.items() if k != "entries"},
                         {k: v for k, v in RELEASE.items() if k != "entries"})

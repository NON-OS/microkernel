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
"""The committed documents under nonos-data/market/catalogue/ are one app
each and agree with the recipes: the services, the set, the ids under the
document's own publisher key, free. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import tempfile
import unittest
from pathlib import Path

from nonos_linux_pack.policy import listing_id, release_id

from .documents import split, write
from .recipes_file import load

ROOT = Path(__file__).resolve().parents[2]
DOCS = ROOT / "nonos-data/market/catalogue"
RECIPES = {r["package"]: r for r in load(ROOT / "userland/capsule_market/linux-catalogue.json", ROOT)}


def documents():
    return [json.loads(p.read_text()) for p in sorted(DOCS.glob("*.json"))]


class Documents(unittest.TestCase):
    def test_split_keeps_the_header_and_one_entry_each(self):
        a, b = {"listing_id": "linux.a"}, {"listing_id": "linux.b"}
        docs = split({"schema_version": 3, "serial": 7, "entries": [a, b]})
        self.assertEqual(docs["linux.b"], {"schema_version": 3, "serial": 7, "entries": [b]})
        with tempfile.TemporaryDirectory() as d:
            got = write({"schema_version": 3, "entries": [a, b]}, Path(d))
            self.assertEqual([p.name for p in got], ["linux.a.json", "linux.b.json"])

    def test_there_is_a_document_for_every_recipe_and_no_other(self):
        names = sorted(d["entries"][0]["listing_id"] for d in documents())
        self.assertEqual(names, sorted(f"linux.{p}" for p in RECIPES))

    def test_every_document_agrees_with_its_recipe(self):
        for doc in documents():
            (e,) = doc["entries"]
            (rel,) = e["releases"]
            recipe = RECIPES[e["listing_id"].removeprefix("linux.")]
            p = bytes.fromhex(rel["ext"]["guest_policy"])
            lid = listing_id(bytes.fromhex(e["publisher_pubkey"]), e["listing_id"])
            self.assertEqual(doc["schema_version"], 3)
            self.assertEqual(int.from_bytes(p[48:56], "little"), recipe["mask"], e["listing_id"])
            self.assertEqual(p[152:217], recipe["calls"], e["listing_id"])
            self.assertEqual((p[56:88], p[88:120]), (lid, release_id(lid, rel["release_id"])))
            self.assertEqual(e["commerce"]["settle_mode"], "free")
            self.assertEqual(rel["package_hash"], rel["manifest_hash"])
            self.assertTrue(rel["package_url"].endswith(f"/{rel['release_id'].replace('@', '-')}.apk"))
            self.assertIn(f"licence {recipe['licence']}", rel["validation"]["note"])
            self.assertIn("installs only in this release's box", rel["validation"]["note"])
            self.assertEqual(rel["publisher_signature"], "")


if __name__ == "__main__":
    unittest.main()

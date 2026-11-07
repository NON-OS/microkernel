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

"""The app images' listings in the catalogue a seal signs: one per committed
entry, published by the operator, unsigned until the seal's signing step,
and a listing id twice refused."""

import json
import shutil
import tempfile
import unittest
from pathlib import Path

from .image_index import add, entries
from .image_listing import ROOT

KEY = "bbeaa6c15c496575134e0c72d8d75d7d6128c9fdf3723761f44026a0adf522f2"
IMAGES = ROOT / "nonos-data/market/images"


class ImageIndex(unittest.TestCase):
    def test_every_committed_entry_makes_an_operator_listing_left_to_the_seal_to_sign(self):
        listed = entries(IMAGES, KEY, 1000, ROOT)
        ids = sorted(e["listing_id"] for e in listed)
        self.assertEqual(ids, ["linux.nonos-image-libreoffice", "linux.nonos-image-telegram-desktop"])
        for e in listed:
            self.assertEqual(e["publisher_pubkey"], KEY)
            self.assertEqual(e["releases"][0]["publisher_signature"], "", "the seal's sign-releases signs it")

    def test_added_after_the_catalogue_and_never_twice(self):
        with tempfile.TemporaryDirectory() as d:
            cat = Path(d) / "catalogue.json"
            cat.write_text(json.dumps({"schema_version": 4, "entries": []}))
            self.assertEqual(add(cat, IMAGES, KEY, 1000, ROOT), 2)
            with self.assertRaises(ValueError):
                add(cat, IMAGES, KEY, 1000, ROOT)

    def test_an_entry_that_makes_no_listing_is_named(self):
        with tempfile.TemporaryDirectory() as d:
            shutil.copy(IMAGES / "linux.nonos-image-libreoffice.json", Path(d) / "a.json")
            (Path(d) / "b.json").write_text("{}")
            with self.assertRaises(ValueError) as e:
                entries(Path(d), KEY, 1000, ROOT)
            self.assertIn("b.json", str(e.exception))

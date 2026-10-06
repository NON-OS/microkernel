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
"""A Qwen tier's listing says its model is downloaded, not that it shipped,
and no listing's own words carry a size or a memory figure: the store's card
shows both from the pins. Run from the repository root:

    python3 -m unittest discover -s tools/nonos_market_catalogue -t tools
"""

import json
import os
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from . import guests

LIST = Path(__file__).resolve().parents[2] / "userland/capsule_market/linux-guests.json"


class Guests(unittest.TestCase):
    def test_a_qwen_tier_is_downloaded_and_a_program_shipped(self):
        for tail in ("qwen-small", "qwen-qwen3-0.6b", "qwen-qwen3-4b", "qwen-coder-32b"):
            self.assertEqual(guests.note(tail), guests.MODEL)
            self.assertNotIn("shipped", guests.note(tail))
        self.assertEqual(guests.note("jq"), guests.SHIPPED)

    def test_the_listing_carries_the_note(self):
        with tempfile.TemporaryDirectory() as d:
            prog = os.path.join(d, "qwenchat")
            with open(prog, "wb") as f:
                f.write(b"\x7fELF")
            listing = os.path.join(d, "guests.json")
            with open(listing, "w") as f:
                json.dump([{"tail": "qwen-small", "name": "Q", "program": prog, "text": "t"}], f)
            with mock.patch.object(guests, "blake3", lambda b: "00" * 32):
                (entry,) = guests.guest_entries(Path(listing), "k", 1)
        self.assertEqual(entry["releases"][0]["validation"]["note"], guests.MODEL)

    def test_no_listing_states_its_own_size_or_memory(self):
        for item in json.loads(LIST.read_text()):
            self.assertTrue(item["tail"].startswith("qwen-"), item["tail"])
            self.assertNotIn("GB", item["text"], item["tail"])
            self.assertNotIn("memory", item["text"], item["tail"])

    def test_the_note_fits_the_detail_pane(self):
        self.assertLessEqual(len(guests.MODEL), len(guests.SHIPPED))


if __name__ == "__main__":
    unittest.main()

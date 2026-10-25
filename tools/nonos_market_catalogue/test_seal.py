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
"""A seal lists every recipe or stops, and only --update-lock writes the
lock it holds to. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import tempfile
import unittest
from pathlib import Path
from unittest import mock

from . import lock, run
from .test_lock import KNOWN, PIN


class Seal(unittest.TestCase):
    def test_a_seal_lists_every_recipe_and_only_update_writes_the_lock(self):
        with tempfile.TemporaryDirectory() as d:
            recipes = Path(d) / "c.json"
            pinned = Path(d) / "c.lock.json"
            listed = [{"listing_id": f"linux.{n}", "releases": [{"package_hash": "ab" * 32}]} for n in "xy"]
            patches = (mock.patch.object(run, "load", return_value=[{"package": "x"}, {"package": "y"}]),
                       mock.patch.object(run, "table", return_value=KNOWN))
            with patches[0], patches[1], mock.patch.object(run, "catalogue_entries", return_value=listed):
                with self.assertRaises(SystemExit):
                    run.catalogue_run(recipes, Path(d), "https://e.org", "v3.20", "x86_64", "11" * 32, 1)
                run.catalogue_run(recipes, Path(d), "https://e.org", "v3.20", "x86_64", "11" * 32, 1, True)
                self.assertEqual(lock.read(pinned), {"x": PIN, "y": PIN})
                self.assertEqual(run.catalogue_run(recipes, Path(d), "https://e.org", "v3.20", "x86_64",
                                                   "11" * 32, 1), listed)
                moved = {**KNOWN, "x": dict(KNOWN["x"], V="1.1-r0")}  # the index moved, the bytes did not
                with mock.patch.object(run, "table", return_value=moved), self.assertRaises(SystemExit):
                    run.catalogue_run(recipes, Path(d), "https://e.org", "v3.20", "x86_64", "11" * 32, 1)
                listed[1]["releases"][0]["package_hash"] = "cd" * 32  # the .apk changed, not its record
                with self.assertRaises(SystemExit):
                    run.catalogue_run(recipes, Path(d), "https://e.org", "v3.20", "x86_64", "11" * 32, 1)
            with patches[0], patches[1], mock.patch.object(run, "catalogue_entries", return_value=listed[:1]), \
                    self.assertRaises(SystemExit):
                run.catalogue_run(recipes, Path(d), "https://e.org", "v3.20", "x86_64", "11" * 32, 1, True)



if __name__ == "__main__":
    unittest.main()

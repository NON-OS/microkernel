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
"""A recipe is refused unless its licence is allowed, its services are real
and leave FILES_USER alone, a dests list comes with NET_ANON, and its set is
market ready. Run: python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import tempfile
import unittest
from pathlib import Path

from nonos_linux_pack.policy import market_ready

from .recipes import UNMEASURED, checked
from .recipes_file import FORMAT, load

ROOT = Path(__file__).resolve().parents[2]
COMMITTED = ROOT / "userland/capsule_market/linux-catalogue.json"
BASE = {"package": "x", "licence": "MIT", "mask": "TTY", "name": "X", "text": "x"}


class Recipes(unittest.TestCase):
    def test_the_committed_catalogue_loads_and_every_app_is_market_ready(self):
        items = load(COMMITTED, ROOT)
        self.assertGreaterEqual(len(items), 17)
        for i in items:
            self.assertTrue(market_ready(i["calls"], i["mask"]), i["package"])

    def test_unmeasured_sets_every_linux_bucket_and_leaves_512_clear(self):
        self.assertEqual(UNMEASURED, bytes([0xFF] * 64 + [0]))

    def test_hostile_recipes_are_refused(self):
        bad = [dict(BASE, licence="Proprietary"), dict(BASE, mask="TTY ROOT"),
               dict(BASE, mask="TTY FILES_USER"), dict(BASE, calls="00" * 65),
               dict(BASE, calls="ff" * 65), dict(BASE, calls="ff" * 64 + "02"),
               dict(BASE, calls="ff" * 64), dict(BASE, extra=1),
               {k: v for k, v in BASE.items() if k != "licence"}]
        for item in bad:
            with self.assertRaises(ValueError, msg=item):
                checked(item, ROOT)

    def test_a_measured_set_is_kept_and_a_dests_list_needs_net_anon(self):
        calls = "calls=" + "ff" * 64 + "00"
        self.assertEqual(checked(dict(BASE, calls=calls), ROOT)["calls"], UNMEASURED)
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "dests").write_text("example.org:443\n\n")
            got = checked(dict(BASE, mask="NET_ANON", dests="dests"), Path(d))
            self.assertTrue(got["dests"])
            with self.assertRaises(ValueError):
                checked(dict(BASE, dests="dests"), Path(d))

    def test_a_package_listed_twice_is_refused(self):
        with tempfile.TemporaryDirectory() as d:
            path = Path(d) / "c.json"
            path.write_text(json.dumps({"format": FORMAT, "apps": [BASE, dict(BASE, name="Y")]}))
            with self.assertRaises(ValueError):
                load(path, ROOT)


if __name__ == "__main__":
    unittest.main()

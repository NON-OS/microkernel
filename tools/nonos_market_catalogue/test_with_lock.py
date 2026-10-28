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
"""An extra a recipe adds is pinned by its index record alone: read back,
held to the fresh index by name like a recipe's package, never in a
recipe's shape, and written beside the recipes in order; each extra is
named once and must be in the signed index. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import tempfile
import unittest
from pathlib import Path

from . import lock
from .listed_with import extras, in_index
from .test_listed_with import BASE
from .test_lock import INDEX, KNOWN, PIN, doc


class WithLock(unittest.TestCase):
    def test_an_extra_s_pin_is_its_index_record_alone(self):
        with tempfile.TemporaryDirectory() as d:
            at = Path(d) / "l.json"
            at.write_text(doc({"x": PIN, "y": INDEX}))
            self.assertEqual(lock.read(at), {"x": PIN, "y": INDEX})
            for bad in (dict(INDEX, V="../1"), dict(INDEX, S=""), dict(INDEX, G="")):
                at.write_text(doc({"x": PIN, "y": bad}))
                with self.assertRaises(ValueError, msg=bad):
                    lock.read(at)

    def test_the_index_must_resolve_every_extra_to_its_pin(self):
        lock.held(KNOWN, {"x": PIN, "y": INDEX}, ["x"], ["y"])
        for pins, adds, said in (({"x": PIN, "y": dict(INDEX, C="Q1other=")}, ["y"], "y: locked"),
                                   ({"x": PIN}, ["y"], "y: not in the lock"),
                                   ({"x": PIN, "z": INDEX}, ["z"], "z: not in the index"),
                                   ({"x": PIN, "y": PIN}, ["y"], "y: an extra pinned as a recipe"),
                                   ({"x": INDEX, "y": INDEX}, ["y"], "x: a recipe pinned as an extra"),
                                   ({"x": PIN, "y": INDEX}, [], "y: locked, no recipe")):
            with self.assertRaises(ValueError, msg=said) as stop:
                lock.held(KNOWN, pins, ["x"], adds)
            self.assertIn(said, str(stop.exception))

    def test_an_update_writes_each_extra_by_its_record_in_order(self):
        found = [{"listing_id": "linux.y", "releases": [{"package_hash": "ab" * 32}]}]
        with tempfile.TemporaryDirectory() as d:
            at = Path(d) / "l.json"
            lock.write(at, KNOWN, ["y"], found, {"y": ("", "")}, ["x"])
            self.assertEqual(lock.read(at), {"x": INDEX, "y": PIN})
            self.assertLess(at.read_text().index('"x"'), at.read_text().index('"y"'))
            lock.write(at, KNOWN, ["y"], found, {"y": ("", "")})
            self.assertEqual(lock.read(at), {"y": PIN})

    def test_each_extra_once_and_held_to_the_signed_index(self):
        recipes = [dict(BASE, **{"with": ["z", "y"]}), dict(BASE, package="y", **{"with": ["z", "w"]})]
        self.assertEqual(extras(recipes), ["w", "z"])
        self.assertEqual(in_index({"w": 1, "z": 2}, ["w", "z"]), {"w": 1, "z": 2})
        with self.assertRaisesRegex(ValueError, "lacks the extras z$"):
            in_index({"w": 1}, ["w", "z"])


if __name__ == "__main__":
    unittest.main()

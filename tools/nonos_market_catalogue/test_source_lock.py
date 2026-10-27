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
"""The sources lock: read whole or refused, written as it is read, and held
by name to each recipe's commit, content and logo. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import tempfile
import unittest
from pathlib import Path

from . import source_lock

BUILT = {"commit": "ab" * 20, "B": "cd" * 32, "S": "10240"}
PIN = dict(BUILT, G="", R="")
MARKS = {"x": ("", "")}


def doc(sources, fmt=source_lock.FORMAT, **more):
    return json.dumps(dict({"format": fmt, "sources": sources}, **more))


class SourceLock(unittest.TestCase):
    def test_a_lock_is_written_and_read_back_whole_or_refused(self):
        with tempfile.TemporaryDirectory() as d:
            at = Path(d) / "l.json"
            self.assertEqual(source_lock.read(at), {}, "no file pins nothing")
            source_lock.write(at, {"x": BUILT}, MARKS)
            self.assertEqual(source_lock.read(at), {"x": PIN})
            for bad in (doc({"x": PIN}, fmt="v0"), doc({"x": PIN}, extra=1), doc([]), doc({"x": BUILT}),
                        doc({"x": dict(PIN, commit="ab" * 4)}), doc({"x": dict(PIN, B="CD" * 32)}),
                        doc({"x": dict(PIN, S="1k")}), doc({"x": dict(PIN, S=1)}), doc({"x": dict(PIN, G="ab")}),
                        doc({"x": dict(PIN, extra="")}), "{", "\xff", doc({"x": PIN}) + " " * (1 << 20)):
                at.write_bytes(bad.encode("latin-1"))
                with self.assertRaises(ValueError, msg=bad[:80]):
                    source_lock.read(at)

    def test_each_recipe_must_build_to_its_pin(self):
        source_lock.held({"x": PIN}, {"x": BUILT}, MARKS)
        for pins, built, said in (({"x": PIN}, {"x": dict(BUILT, commit="ef" * 20)}, "x: commit is"),
                                  ({"x": PIN}, {"x": dict(BUILT, B="ef" * 32)}, "x: B is"),
                                  ({"x": PIN}, {"x": dict(BUILT, S="1")}, "x: S is"),
                                  ({}, {"x": BUILT}, "x: not in the lock"),
                                  ({"x": PIN, "y": PIN}, {"x": BUILT}, "y: locked, no recipe"),
                                  ({"x": dict(PIN, R="ab" * 32)}, {"x": BUILT}, "logo raster")):
            with self.assertRaisesRegex(ValueError, said):
                source_lock.held(pins, built, MARKS)

    def test_the_path_sits_beside_the_recipes(self):
        self.assertEqual(source_lock.path_of(Path("u/linux-catalogue.json")),
                         Path("u/linux-catalogue.sources.lock.json"))

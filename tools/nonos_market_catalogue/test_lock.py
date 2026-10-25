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
"""The lock: read whole or refused, held by name to what the fresh signed
index resolves, and to the .apk bytes each listing signs. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import tempfile
import unittest
from pathlib import Path

from . import lock

INDEX = {"V": "1.0-r0", "C": "Q1abc=", "S": "3"}
PIN = dict(INDEX, B="ab" * 32)
KNOWN = {"x": dict(INDEX, P="x", base="b"), "y": dict(INDEX, P="y", base="b")}


def doc(packages, fmt=lock.FORMAT, **more):
    return json.dumps(dict({"format": fmt, "packages": packages}, **more))


class Lock(unittest.TestCase):
    def test_a_lock_is_read_whole_or_refused(self):
        with tempfile.TemporaryDirectory() as d:
            at = Path(d) / "l.json"
            at.write_text(doc({"x": PIN}))
            self.assertEqual(lock.read(at), {"x": PIN})
            for bad in (doc({"x": PIN}, fmt="v0"), doc({"x": PIN}, extra=1), doc({"X": PIN}),
                        doc({"x": dict(PIN, V="../1")}), doc({"x": dict(PIN, S=3)}), doc({"x": {"V": "1"}}), doc({"x": INDEX}),
                        "{", "\xff"):
                at.write_bytes(bad.encode("latin-1"))
                with self.assertRaises(ValueError, msg=bad):
                    lock.read(at)
            with self.assertRaises(ValueError):
                lock.read(Path(d) / "none.json")
            at.write_text(doc([]))
            with self.assertRaises(TypeError):
                lock.read(at)

    def test_the_index_must_resolve_every_recipe_to_its_pin(self):
        lock.held(KNOWN, {"x": PIN, "y": PIN}, ["x", "y"])
        for pins, names, said in (({"x": dict(PIN, V="0.9-r0"), "y": PIN}, ["x", "y"], "the index has 1.0-r0"),
                                  ({"x": dict(PIN, C="Q1other="), "y": PIN}, ["x", "y"], "x: locked"),
                                  ({"x": PIN}, ["x", "y"], "y: not in the lock"),
                                  ({"x": PIN, "z": PIN}, ["x", "z"], "z: not in the index"),
                                  ({"x": PIN, "y": PIN}, ["x"], "y: locked, no recipe")):
            with self.assertRaises(ValueError) as stop:
                lock.held(KNOWN, pins, names)
            self.assertIn(said, str(stop.exception))

    def test_every_listing_signs_the_apk_its_lock_pins(self):
        listed = [{"listing_id": "linux.x", "releases": [{"package_hash": "ab" * 32}]}]
        lock.bytes_held({"x": PIN}, listed)
        listed[0]["releases"][0]["package_hash"] = "cd" * 32  # same size, another signature member
        with self.assertRaises(ValueError) as stop:
            lock.bytes_held({"x": PIN}, listed)
        self.assertIn("x: the .apk is", str(stop.exception))


if __name__ == "__main__":
    unittest.main()

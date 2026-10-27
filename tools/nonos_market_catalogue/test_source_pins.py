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
"""An asset's pin in the sources lock has exactly its fields, each well
formed, and is read back as written; a moved asset field stops the run by
name. Run: python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import tempfile
import unittest
from pathlib import Path

from . import source_lock

AT = "https://github.com/simplex-chat/simplex-chat/releases/download/v6.4.5/"
BUILT = {"B": "cd" * 32, "S": "10240", "asset": AT + "simplex-chat-ubuntu-22_04-x86_64", "sha256": "ab" * 32,
         "A": "ef" * 32, "sums": AT + "_sha256sums"}
PIN, MARKS = dict(BUILT, G="", R=""), {"x": ("", "")}


class SourcePins(unittest.TestCase):
    def test_an_asset_pin_is_written_and_read_back_whole_or_refused(self):
        with tempfile.TemporaryDirectory() as d:
            at = Path(d) / "l.json"
            source_lock.write(at, {"x": BUILT}, MARKS)
            self.assertEqual(source_lock.read(at), {"x": PIN})
            for bad in (dict(PIN, commit="ab" * 20), {k: v for k, v in PIN.items() if k != "A"},
                        {k: v for k, v in PIN.items() if k != "sums"}, dict(PIN, sha256="ab" * 31),
                        dict(PIN, A="EF" * 32), dict(PIN, asset="http://x.org/a"), dict(PIN, sums="x"),
                        dict(PIN, S="1k"), dict(PIN, sha256=1)):
                at.write_text(json.dumps({"format": source_lock.FORMAT, "sources": {"x": bad}}))
                with self.assertRaises(ValueError, msg=bad):
                    source_lock.read(at)

    def test_each_asset_field_is_held(self):
        source_lock.held({"x": PIN}, {"x": BUILT}, MARKS)
        for field in ("asset", "sha256", "A", "sums", "B", "S"):
            with self.assertRaisesRegex(ValueError, f"x: {field} is"):
                source_lock.held({"x": PIN}, {"x": dict(BUILT, **{field: BUILT[field] + "0"})}, MARKS)
        with self.assertRaisesRegex(ValueError, "x: commit is"):
            source_lock.held({"x": PIN}, {"x": dict(BUILT, commit="ab" * 20)}, MARKS)

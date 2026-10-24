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
"""A recipes file of another format, shape, size or encoding, and a dests
file outside the tree, are refused whole. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import tempfile
import unittest
from pathlib import Path

from .recipes import checked
from .recipes_file import FORMAT, MAX_BYTES, load

BASE = {"package": "x", "licence": "MIT", "mask": "TTY", "name": "X", "text": "x"}


class RecipesFile(unittest.TestCase):
    def refused(self, body: bytes):
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "c.json").write_bytes(body)
            with self.assertRaises(ValueError, msg=body[:60]):
                load(Path(d) / "c.json", Path(d))

    def test_a_file_of_another_format_or_shape_is_refused_whole(self):
        for doc in ([BASE], {"apps": [BASE]}, {"format": "v0", "apps": [BASE]},
                    {"format": FORMAT, "apps": {}}, {"format": FORMAT, "apps": [BASE], "x": 1},
                    {"format": FORMAT, "apps": [BASE, "x"]}, {"format": FORMAT, "apps": [dict(BASE, mask=3)]}):
            self.refused(json.dumps(doc).encode())
        padded = json.dumps({"format": FORMAT, "apps": [BASE]}).encode() + b" " * MAX_BYTES
        for body in (b"\xff\xfe", b"{", b"", padded):
            self.refused(body)

    def test_a_dests_file_outside_the_tree_or_unreadable_is_refused(self):
        with tempfile.TemporaryDirectory() as d:
            (Path(d) / "tree").mkdir()
            (Path(d) / "dests").write_text("example.org:443\n")
            ok = checked(dict(BASE, mask="NET_ANON", dests="dests"), Path(d))
            self.assertTrue(ok["dests"])
            for where in ("../dests", str(Path(d) / "dests"), "missing"):
                with self.assertRaises(ValueError, msg=where):
                    checked(dict(BASE, mask="NET_ANON", dests=where), Path(d) / "tree")


if __name__ == "__main__":
    unittest.main()

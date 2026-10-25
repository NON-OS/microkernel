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

"""No lane table names a key twice: Python keeps the last and drops the rest
without a word, so the table would not say what runs. Nothing here starts
QEMU."""

import ast
import os
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
TABLES = ("lane_devices.py", "lane_disks.py")


def duplicates(source):
    """Every key a dict literal in `source` names more than once."""
    twice = []
    for node in ast.walk(ast.parse(source)):
        if isinstance(node, ast.Dict):
            seen = set()
            for key in node.keys:
                if isinstance(key, ast.Constant):
                    if key.value in seen:
                        twice.append(key.value)
                    seen.add(key.value)
    return twice


class LaneKeys(unittest.TestCase):
    def test_no_lane_table_names_a_key_twice(self):
        for name in TABLES:
            with open(os.path.join(HERE, name)) as f:
                self.assertEqual(duplicates(f.read()), [], name)

    def test_the_check_finds_a_key_named_twice(self):
        self.assertEqual(duplicates('T = {"a": 1, "b": 2, "a": 3}'), ["a"])
        self.assertEqual(duplicates('T = {"a": 1, **{"a": 2}}'), [], "a spread is not a literal key")
        self.assertEqual(duplicates('T = {"a": {"x": 1, "x": 2}}'), ["x"], "nested tables too")


if __name__ == "__main__":
    unittest.main()

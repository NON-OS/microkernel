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

"""Every lane table names each lane once. A dict literal keeps the last of
two equal keys without a word, so a second "usb-tablet" once replaced the
first unseen; the source is read here so a repeat fails instead. Nothing here
starts QEMU."""

import ast
import os
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))


def repeated(path):
    """(table, key) for every key a dict literal in `path` names twice."""
    tree = ast.parse(open(path, encoding="utf-8").read(), path)
    out = []
    for node in ast.walk(tree):
        if isinstance(node, ast.Dict):
            keys = [k.value for k in node.keys if isinstance(k, ast.Constant)]
            out += sorted({k for k in keys if keys.count(k) > 1})
    return out


class LaneKeys(unittest.TestCase):
    def test_no_lane_table_names_a_lane_twice(self):
        for name in sorted(os.listdir(HERE)):
            if name.startswith("lane") and name.endswith(".py"):
                self.assertEqual(repeated(os.path.join(HERE, name)), [], name)

    def test_a_repeat_is_found(self):
        path = os.path.join(HERE, "lane_devices.py")
        text = open(path, encoding="utf-8").read().replace(
            '    "usb-kbd":', '    "usb-ccid": "x",\n    "usb-kbd":', 1)
        tree = ast.parse(text)
        keys = [k.value for n in ast.walk(tree) if isinstance(n, ast.Dict)
                for k in n.keys if isinstance(k, ast.Constant)]
        self.assertGreater(keys.count("usb-ccid"), 1)


if __name__ == "__main__":
    unittest.main()

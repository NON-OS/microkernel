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
"""The tier the seal lays on the release stick is the one setup, the store
and the fetcher call the stick tier (STICK_TIER in need.rs), and it is a
pinned tier."""

import os
import re
import unittest

from .pins import pins

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))


def _read(rel, pattern):
    text = open(os.path.join(ROOT, rel)).read()
    found = re.findall(pattern, text, re.M)
    if len(found) != 1:
        raise AssertionError(f"{rel}: {pattern} found {len(found)} times")
    return found[0]


class Stick(unittest.TestCase):
    def test_the_seal_and_the_userland_name_the_same_tier(self):
        seal = _read("tools/nonos_seal/media.py", r'^STICK_TIER = "([\w.-]+)"$')
        need = _read("userland/capsule_model_fetch/src/need.rs",
                     r'^pub const STICK_TIER: &str = "([\w.-]+)";$')
        self.assertEqual(seal, need)
        self.assertIn(need, pins())


if __name__ == "__main__":
    unittest.main()

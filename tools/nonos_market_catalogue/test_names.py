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
"""Every name an untrusted index or recipe supplies is held to Alpine's
spelling before it reaches a URL, a cache path or a file name. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest

from .names import index_record, listing, package, version

REC = {"P": "git", "V": "2.45.4-r0", "L": "GPL-2.0-only", "base": "https://example.org/x", "S": "3281791"}


class Names(unittest.TestCase):
    def test_alpine_s_own_spellings_pass(self):
        for name in ("git", "py3-pip", "gtk+3.0", "lua5.4", "a"):
            self.assertEqual(package(name), name)
        for v in ("1.0-r0", "2024.05.18-r0", "6.4_p20240420-r2", "0.18.3.3-r1"):
            self.assertEqual(version(v), v)
        for lid in ("linux.helix", "nonos.app.text_editor", "community.some-app"):
            self.assertEqual(listing(lid), lid)

    def test_traversal_separators_and_odd_types_are_refused(self):
        for bad in ("../x", "a/b", ".x", "", "-x", "Git", "a b", "a\x00", "x" * 65, None, 7, ["git"]):
            with self.assertRaises(ValueError, msg=repr(bad)):
                package(bad)
        for bad in ("../1", "1/2", ".1", "", "r0", "1\n", None, 1.0):
            with self.assertRaises(ValueError, msg=repr(bad)):
                version(bad)
        for bad in ("linux.../x", "linux./x", "nonos.x", "linux.", "linux.a/b", "linux..a", "kali.x", None):
            with self.assertRaises(ValueError, msg=repr(bad)):
                listing(bad)

    def test_an_index_record_with_a_bad_field_is_refused(self):
        self.assertIs(index_record(REC), REC)
        for bad in (dict(REC, V="../../etc"), dict(REC, P="a/b"), {k: v for k, v in REC.items() if k != "V"},
                    dict(REC, L=["MIT"]), {k: v for k, v in REC.items() if k != "base"}, "git", None):
            with self.assertRaises((ValueError, TypeError), msg=repr(bad)):
                index_record(bad)


if __name__ == "__main__":
    unittest.main()

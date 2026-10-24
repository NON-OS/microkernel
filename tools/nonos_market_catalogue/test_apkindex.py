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
"""The index is read as the device reads it (install/index.rs,
get/index.rs): APKINDEX entries joined, lines as Rust's, a record only with
a name and a version, the listed package the first of its exact name, never
a provider. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest

from .apk_fixture import tar
from .apkindex import listed, records, rest, text
from .rust_text import lines, trim

# Alpine v3.20: gvim, in main, provides vim ahead of vim itself.
GVIM = b"P:gvim\nV:9.1-r0\np:vim=9.1-r0\n\nP:vim\nV:9.1-r0\nL:Vim\nS:1442\n\n"


class Index(unittest.TestCase):
    def test_the_listed_package_is_its_own_name_never_a_provider(self):
        recs = records(GVIM)
        self.assertEqual(listed(recs, "vim")["L"], "Vim")
        self.assertIsNone(listed(records(b"P:only\nV:1\np:ghost=1\n\n"), "ghost"))

    def test_the_first_of_a_name_wins_main_before_community(self):
        main, community = tar([("APKINDEX", b"P:a\nV:1\nS:1\n\n")]), tar([("APKINDEX", b"P:a\nV:2\nS:2\n\n")])
        self.assertEqual(listed(records(text(main) + text(community)), "a")["V"], "1")

    def test_a_record_needs_a_name_and_a_version(self):
        self.assertEqual(records(b"P:a\n\nV:1\n\nP:b\nV:2\n\n"), [{"P": "b", "V": "2"}])

    def test_every_entry_ending_in_apkindex_is_joined_and_only_those(self):
        index = tar([("DESCRIPTION", b"P:d\nV:1\n\n"), ("APKINDEX", b"P:a\nV:1\n\n"), ("x/APKINDEX", b"P:b\nV:1\n\n")])
        self.assertEqual([r["P"] for r in records(text(index))], ["a", "b"])

    def test_lines_and_fields_are_cut_as_rust_cuts_them(self):
        self.assertEqual(lines("a\r\nb\n\nc\n"), ["a", "b", "", "c"])
        self.assertEqual(lines("a\r\r\n"), ["a\r"])
        self.assertEqual(records(b"P:a\r\nV:1\r\n\r\n"), [{"P": "a", "V": "1"}])
        self.assertEqual((rest("P:x"), rest("P"), rest("Péx")), ("x", "", ""))
        self.assertEqual(trim("\t  x  "), "x")
        self.assertEqual(trim("\x1fx\x1c"), "\x1fx\x1c")

    def test_a_key_line_needs_its_letter_first_and_others_are_ignored(self):
        self.assertEqual(records(b"P:a\nV:1\nX:q\nAP:b\n\n"), [{"P": "a", "V": "1"}])


if __name__ == "__main__":
    unittest.main()

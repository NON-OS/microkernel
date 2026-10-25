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
"""The tar walk reads every archive the way install/tar.rs does: archives
end to end, a size that is not octal or runs past the end stops the walk,
pax and GNU long names apply to the next entry only, and names lose `./`
and a trailing `/`. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest

from .apk_fixture import tar
from .tar_fixture import head, pax, rec
from .tarwalk import files, walk


class Walk(unittest.TestCase):
    def test_archives_end_to_end_are_one_walk(self):
        both = tar([("a", b"1")]) + tar([("b", b"2")])
        self.assertEqual([e.name for e in files(both)], [b"a", b"b"])

    def test_a_bad_or_overlong_size_stops_the_walk(self):
        bad = bytearray(rec(b"b", b"0", b"x"))
        bad[124:136] = b"zzzzzzzzzzz\0"
        self.assertEqual([e.name for e in walk(rec(b"a", b"0", b"1") + bytes(bad) + rec(b"c", b"0"))], [b"a"])
        self.assertEqual(walk(head(b"a", b"0", 4096) + b"short"), [])

    def test_long_names_apply_to_the_next_entry_only(self):
        data = rec(b"p", b"x", pax("path", "usr/bin/long-name")) + rec(b"short", b"0", b"1") + rec(b"after", b"0")
        self.assertEqual([e.name for e in walk(data)], [b"usr/bin/long-name", b"after"])
        gnu = rec(b"././@LongLink", b"L", b"usr/bin/gnu\0") + rec(b"x", b"0")
        self.assertEqual([e.name for e in walk(gnu)], [b"usr/bin/gnu"])

    def test_names_are_cut_as_the_installer_cuts_them(self):
        data = rec(b"./usr/bin/a", b"0") + rec(b"usr/share/d/", b"5") + rec(b"bin/x", b"0", prefix=b"usr")
        self.assertEqual([(e.name, e.kind) for e in walk(data)],
                         [(b"usr/bin/a", "file"), (b"usr/share/d", "dir"), (b"usr/bin/x", "file")])

    def test_a_device_is_dropped_and_forgets_a_pending_name(self):
        data = rec(b"p", b"x", pax("path", "renamed")) + rec(b"dev", b"3") + rec(b"kept", b"0")
        self.assertEqual([e.name for e in walk(data)], [b"kept"])
        data = rec(b"g", b"g", pax("x", "y")) + rec(b"p", b"x", pax("path", "renamed")) + rec(b"f", b"0")
        self.assertEqual([e.name for e in walk(data)], [b"renamed"])

    def test_a_pax_length_too_short_to_hold_its_record_ends_the_records(self):
        for body in (b"0 path=x\n", b"2 path=x\n", b"99 path=x\n"):
            data = rec(b"p", b"x", body) + rec(b"kept", b"0")
            self.assertEqual([e.name for e in walk(data)], [b"kept"], body)

    def test_a_pax_length_reads_as_the_installer_reads_it(self):
        record = b" path=renamed\n"
        zeros = b"0" * 19 + str(21 + len(record)).encode() + record  # 21 digits, as Rust takes it
        self.assertEqual([e.name for e in walk(rec(b"p", b"x", zeros) + rec(b"f", b"0"))], [b"renamed"])
        huge = b"9" * 5000 + b" path=x\n"  # overflows a usize: the records end, nothing raises
        self.assertEqual([e.name for e in walk(rec(b"p", b"x", huge) + rec(b"kept", b"0"))], [b"kept"])

    def test_only_regular_files_are_files(self):
        data = rec(b"f", b"0", b"1") + rec(b"s", b"2") + rec(b"h", b"1") + rec(b"c", b"7", b"2")
        self.assertEqual([e.name for e in files(data)], [b"f", b"c"])


if __name__ == "__main__":
    unittest.main()

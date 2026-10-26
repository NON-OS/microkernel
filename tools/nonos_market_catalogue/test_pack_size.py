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
"""A package whose pack would pass the format's bounds is refused when its
listing is built (pack_size.py). The length is the one nonos_linux_pack's
build writes: 9288 was read off `build` for the same files. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest
from unittest import mock

from . import pack_size
from .apk_fixture import ELF, tar

# usr/bin/vim with its trailer, usr/share/a, usr/share/empty.
SMALL = [(".PKGINFO", b"pkg"), ("usr/bin/vim", ELF), ("usr/share/a", b"x" * 5000),
         ("usr/share/empty", b"")]


class PackSize(unittest.TestCase):
    def test_the_length_is_the_one_build_writes(self):
        self.assertEqual(pack_size.pack_len(tar(SMALL)), 9288)

    def test_the_last_copy_of_a_name_is_the_one_counted(self):
        twice = SMALL + [("usr/share/a", b"x")]
        self.assertLess(pack_size.pack_len(tar(twice)), 9288)
        self.assertEqual(len(pack_size.sizes(tar(twice))), 4)

    def test_a_pack_up_to_the_bound_is_listed(self):
        pack_size.check(tar([("usr/share/a", b"x" * (pack_size.MAX_PACK - 4096))]))

    def test_a_pack_past_the_bound_is_refused_not_cut(self):
        with self.assertRaisesRegex(ValueError, "past the 64 MiB"):
            pack_size.check(tar([("usr/share/a", b"x" * (pack_size.MAX_PACK - 4095))]))

    def test_more_files_than_a_pack_holds_are_refused(self):
        with mock.patch.object(pack_size, "MAX_ENTRIES", 4):
            pack_size.check(tar(SMALL))
            with self.assertRaisesRegex(ValueError, "more than 4 files"):
                pack_size.check(tar(SMALL + [("usr/share/b", b"b")]))


if __name__ == "__main__":
    unittest.main()

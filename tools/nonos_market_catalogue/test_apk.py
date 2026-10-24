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
"""The program a catalogue policy names is the one capsule_linux records
(install/record/program.rs): usr/bin/<name> if the package has that regular
file, ELF or not, else its first ELF under usr/bin; the last copy of a name
is the one that lands. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest

from .apk import program
from .apk_fixture import ELF, tar


class Program(unittest.TestCase):
    def test_the_package_s_own_name_wins_over_an_earlier_program(self):
        files = tar([("usr/bin/aaa", ELF + b"a"), ("usr/bin/tool", ELF + b"t")])
        self.assertEqual(program(files, "tool"), ("usr/bin/tool", ELF + b"t"))

    def test_else_the_first_elf_in_archive_order(self):
        files = tar([("usr/share/x", ELF), ("usr/bin/zz", ELF + b"z"), ("usr/bin/hx", ELF + b"h")])
        self.assertEqual(program(files, "helix")[0], "usr/bin/zz")

    def test_a_script_ahead_of_the_first_elf_is_passed_over(self):
        files = tar([("usr/bin/helper", b"#!/bin/sh\n"), ("usr/bin/hx", ELF + b"h")])
        self.assertEqual(program(files, "helix")[0], "usr/bin/hx")

    def test_an_own_name_that_is_not_an_elf_is_refused_not_passed_over(self):
        files = tar([("usr/bin/helix", b"#!/bin/sh\nexec hx\n"), ("usr/bin/hx", ELF)])
        with self.assertRaises(ValueError):
            program(files, "helix")

    def test_a_leading_dot_slash_is_the_same_name(self):
        files = tar([("usr/bin/aaa", ELF + b"a"), ("./usr/bin/tool", ELF + b"t")])
        self.assertEqual(program(files, "tool"), ("usr/bin/tool", ELF + b"t"))

    def test_the_last_copy_of_the_name_is_the_one_hashed(self):
        files = tar([("usr/bin/tool", ELF + b"first"), ("usr/bin/tool", ELF + b"last")])
        self.assertEqual(program(files, "tool")[1], ELF + b"last")

    def test_links_and_programs_elsewhere_are_not_programs(self):
        files = tar([("usr/lib/x/hx", ELF), ("usr/bin/hx", ELF + b"h")], {"usr/bin/view": "/usr/bin/hx"})
        self.assertEqual(program(files, "view")[0], "usr/bin/hx")

    def test_a_package_with_no_program_has_none(self):
        self.assertIsNone(program(tar([("usr/share/doc/README", b"hi")]), "doc"))
        self.assertIsNone(program(b"", "doc"))


if __name__ == "__main__":
    unittest.main()

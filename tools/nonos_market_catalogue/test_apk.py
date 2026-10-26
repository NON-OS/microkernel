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


class ByBytes(unittest.TestCase):
    """A recipe's `program`: install/market/choose.rs records the usr/bin
    name whose last copy is the policy's bytes, its own name first."""

    def test_the_wallet_among_the_daemon_and_tools(self):
        files = tar([("usr/bin/monero-blockchain-ancestry", ELF + b"a"),
                     ("usr/bin/monero-wallet-cli", ELF + b"w"), ("usr/bin/monerod", ELF + b"d")])
        self.assertEqual(program(files, "monero", "usr/bin/monero-wallet-cli"),
                         ("usr/bin/monero-wallet-cli", ELF + b"w"))

    def test_missing_scripts_and_paths_outside_usr_bin_are_refused(self):
        files = tar([("usr/bin/s", b"#!/bin/sh\n"), ("usr/lib/w", ELF + b"w")])
        for wanted in ("usr/bin/absent", "usr/bin/s", "usr/lib/w"):
            with self.assertRaises(ValueError, msg=wanted):
                program(files, "p", wanted)

    def test_bytes_the_device_would_find_under_another_name_first_are_refused(self):
        files = tar([("usr/bin/a", ELF + b"x"), ("usr/bin/b", ELF + b"x")])
        with self.assertRaises(ValueError):
            program(files, "p", "usr/bin/b")
        self.assertEqual(program(files, "p", "usr/bin/a")[0], "usr/bin/a")
        self.assertEqual(program(files, "b", "usr/bin/b")[0], "usr/bin/b", "the own name wins on the device too")

    def test_the_last_copy_is_the_one_named(self):
        files = tar([("usr/bin/w", ELF + b"old"), ("usr/bin/w", ELF + b"new")])
        self.assertEqual(program(files, "p", "usr/bin/w")[1], ELF + b"new")


if __name__ == "__main__":
    unittest.main()

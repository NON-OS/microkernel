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
"""The program a catalogue policy names is the one capsule_linux records:
usr/bin/<name> when the package has it, else its first ELF under usr/bin.
Run from the repository root:

    python3 -m unittest discover -s tools/nonos_market_catalogue -t tools
"""

import gzip
import unittest
from unittest import mock

from . import apk
from .apk_fixture import ELF, make


class Program(unittest.TestCase):
    def test_the_package_s_own_name_wins_over_an_earlier_program(self):
        raw = make([("usr/bin/aaa", ELF + b"a"), ("usr/bin/tool", ELF + b"t")])
        self.assertEqual(apk.program(raw, "tool"), ("usr/bin/tool", ELF + b"t"))

    def test_else_the_first_elf_in_archive_order(self):
        raw = make([("usr/bin/zz", ELF + b"z"), ("usr/bin/hx", ELF + b"h")])
        self.assertEqual(apk.program(raw, "helix")[0], "usr/bin/zz")

    def test_scripts_links_and_programs_elsewhere_are_not_programs(self):
        raw = make([("usr/bin/helix", b"#!/bin/sh\nexec hx\n"), ("usr/lib/x/hx", ELF),
                    ("usr/bin/hx", ELF + b"h")], {"usr/bin/view": "vim"})
        self.assertEqual(apk.program(raw, "helix")[0], "usr/bin/hx")

    def test_a_package_with_no_program_has_none(self):
        self.assertIsNone(apk.program(make([("usr/share/doc/README", b"hi")]), "doc"))

    def test_the_control_members_are_read_but_never_chosen(self):
        names = [n for n, _ in apk.files(make([("usr/bin/a", ELF)]))]
        self.assertEqual(names, [".SIGN.RSA.test.rsa.pub", ".PKGINFO", "usr/bin/a"])


class Hostile(unittest.TestCase):
    def test_a_package_cut_inside_a_member_is_refused(self):
        raw = make([("usr/bin/a", ELF)])
        with self.assertRaises(ValueError):
            apk.files(raw[:-9])

    def test_a_package_that_inflates_past_the_limit_is_refused(self):
        raw = gzip.compress(bytes(4097)) + gzip.compress(bytes(10))
        with mock.patch.object(apk, "LIMIT", 4096), self.assertRaises(ValueError):
            apk.inflate(raw)
        with mock.patch.object(apk, "LIMIT", 4100), self.assertRaises(ValueError):
            apk.inflate(raw)
        with mock.patch.object(apk, "LIMIT", 4107):
            self.assertEqual(len(apk.inflate(raw)), 4107)

    def test_garbage_is_refused_not_read(self):
        with self.assertRaises(ValueError):
            apk.files(b"\x1f\x8b" + bytes(40))
        self.assertIsNone(apk.program(gzip.compress(b"no tar header " * 64), "x"))


if __name__ == "__main__":
    unittest.main()

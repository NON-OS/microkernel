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
"""The boot census verdict; nothing here starts QEMU. Run with
python3 -m unittest discover -s scripts -p 'test_census.py'."""

import unittest

from bootmatrix import census

VM = """\
[CENSUS] taken 1012 ms after init's spawns
[CENSUS] battery absent expected=0 up=0 why=firmware declares none
[CENSUS] storage up expected=1 up=1
[CENSUS] summary up=6 absent=5 held=0 missing=0 verdict=green
"""
RED = """\
[CENSUS] taken 30004 ms after init's spawns
[CENSUS] storage missing expected=1 up=0 why=driver_virtio_blk
[CENSUS] summary up=5 absent=5 held=0 missing=1 verdict=red
"""


class Census(unittest.TestCase):
    def test_a_green_vm_census_passes(self):
        self.assertEqual(census.verdict(VM), [])

    def test_a_missing_device_fails_with_its_reason(self):
        self.assertEqual(census.verdict(RED), ["census: storage missing: driver_virtio_blk"])

    def test_the_last_census_counts(self):
        self.assertEqual(census.verdict(RED + VM), [])
        self.assertEqual(len(census.verdict(VM + RED)), 1)

    def test_no_census_fails(self):
        self.assertEqual(census.verdict("[INIT] Capsules spawned\n"), ["no [CENSUS] summary line"])


if __name__ == "__main__":
    unittest.main()

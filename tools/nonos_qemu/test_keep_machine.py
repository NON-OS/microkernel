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
"""--keep-machine: the settle boot of scripts/amnesic_boot_check.sh runs on the
variable store and system partition the main boot left, so the main boot's
writes are the ones judged, and are never laid fresh in between."""

import os
import sys
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import disk  # noqa: E402


class KeepMachine(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.mkdtemp()
        self.pristine = os.path.join(self.dir, "pristine.fd")
        with open(self.pristine, "wb") as f:
            f.write(b"pristine")

    def test_a_new_boot_takes_a_new_copy_of_the_pristine_store(self):
        store = disk.vars_store(self.dir, self.pristine)
        with open(store, "wb") as f:
            f.write(b"written by the main boot")
        disk.vars_store(self.dir, self.pristine)
        self.assertEqual(open(store, "rb").read(), b"pristine")

    def test_the_kept_store_is_the_one_the_main_boot_wrote(self):
        store = disk.vars_store(self.dir, self.pristine)
        with open(store, "wb") as f:
            f.write(b"written by the main boot")
        self.assertEqual(disk.vars_store(self.dir, self.pristine, keep=True), store)
        self.assertEqual(open(store, "rb").read(), b"written by the main boot")

    def test_keeping_what_no_boot_left_is_refused(self):
        with self.assertRaises(SystemExit):
            disk.vars_store(self.dir, self.pristine, keep=True)
        with self.assertRaises(SystemExit):
            disk.prepare("/no/image", os.path.join(self.dir, "esp"), "/no/data", False, [], keep=True)

    def test_the_kept_partition_is_not_laid_again(self):
        esp = os.path.join(self.dir, "esp")
        os.makedirs(esp)
        with open(os.path.join(esp, "written"), "w") as f:
            f.write("by the main boot")
        laid, kept = [], disk.esp
        disk.esp = lambda source, into: laid.append(into)
        try:
            old = disk.needs_copy, disk.drop_gpt
            disk.needs_copy, disk.drop_gpt = (lambda *a: False), (lambda *a: False)
            disk.prepare("/no/image", esp, "/no/data", False, [], keep=True)
            self.assertEqual(laid, [])
            disk.prepare("/no/image", esp, "/no/data", False, [])
            self.assertEqual(laid, [esp])
        finally:
            disk.esp = kept
            disk.needs_copy, disk.drop_gpt = old
        self.assertTrue(os.path.exists(os.path.join(esp, "written")))


if __name__ == "__main__":
    unittest.main()

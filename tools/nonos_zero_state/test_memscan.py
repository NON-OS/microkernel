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

"""The secret counter's own proofs: a match across a slice edge is counted
once, every match is counted, and a dump is gone after it is read. Run with
`python3 -m unittest nonos_zero_state.test_memscan` from tools/."""

import os
import tempfile
import unittest
from unittest import mock

from . import memscan


class Count(unittest.TestCase):
    def file(self, data):
        f = tempfile.NamedTemporaryFile(delete=False)
        f.write(data)
        f.close()
        self.addCleanup(lambda: os.path.exists(f.name) and os.remove(f.name))
        return f.name

    def test_every_offset_against_every_slice_edge(self):
        secret = memscan.marker()
        for slice_len in (7, 8, 13, 64):
            with mock.patch.object(memscan, "SLICE", slice_len):
                for at in range(0, 80):
                    path = self.file(b"\0" * at + secret.encode() + b"\xff" * 30)
                    self.assertEqual(memscan.count(path, secret), 1, (slice_len, at))

    def test_many_and_none(self):
        secret = memscan.marker()
        with mock.patch.object(memscan, "SLICE", 11):
            self.assertEqual(memscan.count(self.file((secret.encode() + b"x") * 9), secret), 9)
            self.assertEqual(memscan.count(self.file(secret[:-1].encode() * 9), secret), 0)

    def test_a_dump_is_removed_once_read(self):
        secret = memscan.marker()
        path = self.file(secret.encode())
        self.assertEqual(memscan.count_and_drop(path, secret), 1)
        self.assertFalse(os.path.exists(path))

    def test_wide_forms_are_counted(self):
        secret = memscan.marker()
        for enc in ("utf-16-le", "utf-32-le"):
            with mock.patch.object(memscan, "SLICE", 13):
                for at in range(0, 40):
                    path = self.file(b"\x01" * at + secret.encode(enc) + b"\x01" * 9)
                    self.assertEqual(memscan.count(path, secret), 1, (enc, at))

    def test_markers_differ_and_are_letters(self):
        a, b = memscan.marker(), memscan.marker()
        self.assertNotEqual(a, b)
        self.assertTrue(a.isalpha() and a.islower() and len(a) == 18)


if __name__ == "__main__":
    unittest.main()

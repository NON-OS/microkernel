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
"""A gzip file is cut into members as the installer's inflater cuts it: at
most 64, inflating to at most the limit together, nothing between them, and
output bounded before it is made. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import gzip
import tracemalloc
import unittest

from . import gz


class Members(unittest.TestCase):
    def test_members_carry_their_byte_ranges(self):
        a, b = gzip.compress(b"one"), gzip.compress(b"two")
        got = gz.members(a + b)
        self.assertEqual([(m.start, m.end, m.body) for m in got], [(0, len(a), b"one"), (len(a), len(a + b), b"two")])

    def test_cut_garbage_trailing_and_empty_input_are_refused(self):
        whole = gzip.compress(b"x" * 100)
        for raw in (whole[:-3], whole + b"\x00", b"\x1f\x8b" + bytes(20), b"", b"plain text"):
            with self.assertRaises(ValueError, msg=raw[:12]):
                gz.members(raw)

    def test_at_most_sixty_four_members(self):
        one = gzip.compress(b"")
        self.assertEqual(len(gz.members(one * gz.MAX_MEMBERS)), gz.MAX_MEMBERS)
        with self.assertRaises(ValueError):
            gz.members(one * (gz.MAX_MEMBERS + 1))

    def test_the_limit_counts_every_member_together(self):
        raw = gzip.compress(bytes(3000)) + gzip.compress(bytes(3000))
        self.assertEqual(len(gz.members(raw, limit=6000)), 2)
        with self.assertRaises(ValueError):
            gz.members(raw, limit=5999)

    def test_a_bomb_is_refused_without_being_inflated(self):
        bomb = gzip.compress(bytes(64 << 20))
        tracemalloc.start()
        try:
            with self.assertRaises(ValueError):
                gz.members(bomb, limit=1 << 16)
            peak = tracemalloc.get_traced_memory()[1]
        finally:
            tracemalloc.stop()
        self.assertLess(peak, 4 << 20)


if __name__ == "__main__":
    unittest.main()

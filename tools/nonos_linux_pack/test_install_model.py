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

"""The install-time model's arithmetic, on the numbers the track notes
quote, so a change to the model changes the notes' table on purpose."""

import unittest

from .install_model import KVM, Link, image, today

ANYONE, CLEAR = Link(0.5, 1.5), Link(10.0, 0.05)
TELEGRAM = dict(gets=253, apk=136_279_519, installed=377_430_016)


class Model(unittest.TestCase):
    def test_todays_telegram_is_the_notes_row(self):
        t = today(ANYONE, KVM, **TELEGRAM)
        self.assertAlmostEqual(t["total"], 713, delta=1)
        c = today(CLEAR, KVM, **TELEGRAM)
        self.assertAlmostEqual(c["total"], 80, delta=1)

    def test_a_cache_hit_moves_nothing(self):
        r = image(ANYONE, KVM, size=100 * 10**6, chunk=4 << 20, raw=377 * 10**6, installed=377 * 10**6,
                  cached=24)
        self.assertEqual(r["net"], 0)

    def test_width_divides_rounds_and_bytes(self):
        one = image(CLEAR, KVM, size=100 << 20, chunk=4 << 20, raw=1, installed=1, width=1)
        four = image(CLEAR, KVM, size=100 << 20, chunk=4 << 20, raw=1, installed=1, width=4)
        self.assertLess(four["net"], one["net"] / 3)

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


"""No line in any log may say ESCAPED, in any shape, and no [ATTACK] line
may be unreadable; a guest's summary of a held run is no escape."""

import unittest

from .judge import judge
from .test_judge import with_capsule


class Escapes(unittest.TestCase):
    def test_an_escape_in_any_shape_fails(self):
        for line in ("[ATTACK] cap-escape ESCAPED\n", "[ATTACK] Cap-Escape ESCAPED: x\n",
                     "[ATTACK] cap-escape ESC\nAPED: x\n", "[GUEST] fs ESCAPED the root\n",
                     "[ATTACK] fault-containment ESCAPED: x\n", "[ATTACK] cap-escape refu\n",
                     "[GUEST] fs BROKEN: 12 tried, 1 escaped\n"):
            self.assertTrue(judge(with_capsule(lambda t: t + line)).bad, line)

    def test_a_guests_held_summary_is_no_escape(self):
        logs = with_capsule(lambda t: t + "[GUEST] fs held: 12 tried, 0 escaped\n")
        self.assertEqual(judge(logs).bad, [])

if __name__ == "__main__":
    unittest.main()

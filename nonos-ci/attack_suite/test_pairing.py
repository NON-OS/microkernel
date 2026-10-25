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


"""The kernel's line beside each attempt."""

import unittest

from .judge import judge
from .sample import passing
from .test_judge import with_capsule

class Pairing(unittest.TestCase):
    def test_capsule_lines_carry_the_attackers_own_deny(self):
        caps = [a for a in judge(passing()).attempts if a.name == "cap-escape"]
        self.assertTrue(caps)
        self.assertTrue(all(a.kernel and a.kernel.startswith("[KLOG WARN] [CAP-DENY] pid=41 ") for a in caps))

    def test_another_pids_deny_is_not_borrowed(self):
        logs = with_capsule(lambda t: t.replace("[CAP-DENY] pid=41", "[CAP-DENY] pid=7"))
        self.assertTrue(all(a.kernel is None for a in judge(logs).attempts if a.name == "cap-escape"))

    def test_a_deny_before_the_previous_attempt_is_not_borrowed(self):
        log = ("[ATTACK-START] capsule pid 41\n[CAP-DENY] pid=41 syscall=MkIpcSend(1)\n"
               "[ATTACK] cap-escape refused: a\n[ATTACK] cap-escape refused: b\n")
        a, b = [x for x in judge([log]).attempts if x.name == "cap-escape"]
        self.assertIsNotNone(a.kernel)
        self.assertIsNone(b.kernel)


if __name__ == "__main__":
    unittest.main()

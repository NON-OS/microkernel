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


"""The attacker's fault, which only the kernel's lines in the capsule boot's
own log can judge."""

import unittest

from .judge import judge
from .mutants import FORGED
from .sample import FAULTED, passing
from .test_judge import with_capsule


def fault(logs):
    return [a for a in judge(logs).attempts if a.name == "fault-containment"]


class Fault(unittest.TestCase):
    def test_the_fault_rests_on_the_trap_line(self):
        (f,) = fault(passing())
        self.assertEqual(f.verdict, "refused")
        self.assertTrue(f.kernel.startswith("[TRAP PF] cpl=3"))

    def test_another_boots_lines_do_not_follow_the_trap(self):
        cut = lambda t: t[:t.index("[TRAP WALK]")]  # noqa: E731
        (f,) = fault(with_capsule(cut))
        self.assertEqual(f.verdict, "ESCAPED")

    def test_another_boots_panic_is_not_the_capsule_boots(self):
        attest, capsule, linux = passing()
        (f,) = fault([attest, capsule, linux + "KERNEL PANIC at src/mm/x.rs:1\n"])
        self.assertEqual(f.verdict, "refused")
        (f,) = fault(with_capsule(lambda t: t + "KERNEL PANIC at src/mm/x.rs:1\n"))
        self.assertEqual(f.verdict, "ESCAPED")

    def test_a_line_claiming_the_refusal_is_not_one(self):
        logs = with_capsule(lambda t: t.replace(FAULTED, FORGED + "[NET] up\n"))
        self.assertEqual(fault(logs), [])
        self.assertIn("fault-containment: never attempted", judge(logs).bad)

    def test_a_trap_quoted_inside_a_line_is_not_one(self):
        logs = with_capsule(lambda t: t.replace("[TRAP PF]", "[NET] said [TRAP PF]"))
        (f,) = fault(logs)
        self.assertEqual(f.verdict, "ESCAPED")


if __name__ == "__main__":
    unittest.main()

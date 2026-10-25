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


"""The verdict over the synthetic sample logs, one per boot."""

import unittest

from .judge import judge
from .rules import ATTACKS
from .sample import passing


def with_capsule(f):
    attest, capsule, linux = passing()
    return [attest, f(capsule), linux]


class Judge(unittest.TestCase):
    def test_the_sample_passes_and_proves_every_row(self):
        r = judge(passing())
        self.assertEqual(r.bad, [])
        self.assertTrue(all(not missing for _, missing in r.rows))
        self.assertEqual({a.name for a in r.attempts}, set(ATTACKS))

    def test_an_empty_log_fails(self):
        self.assertTrue(all(judge(logs).bad for logs in ("", ["\n\n", "   "], [])))

    def test_one_escape_among_refusals_fails(self):
        logs = with_capsule(lambda t: t + "[ATTACK] cap-escape ESCAPED: MkIrqBind returned 0\n")
        self.assertIn("cap-escape: ESCAPED: MkIrqBind returned 0", judge(logs).bad)

    def test_a_refusal_for_another_reason_proves_no_row(self):
        logs = with_capsule(lambda t: t.replace("EPERM (1)", "ENOENT (2)"))
        bad = judge(logs).bad
        self.assertTrue(any("cap-escape: Network" in b for b in bad), bad)

    def test_an_unknown_or_dropped_attack_is_a_note_not_a_pass(self):
        r = judge(with_capsule(lambda t: t + "[ATTACK] made-up refused: x\n[ATTACK] chainload refused: y\n"))
        self.assertEqual(r.bad, [])
        self.assertTrue(any(n.startswith("made-up:") for n in r.notes))
        self.assertTrue(any(n.startswith("chainload:") for n in r.notes))

    def test_one_boot_judged_alone_is_judged_for_its_attacks_alone(self):
        attest, capsule, linux = passing()
        self.assertEqual(judge([capsule], ["capsule"]).bad, [])
        self.assertEqual(judge([capsule], ["capsule"]).boots, ["capsule"])
        self.assertTrue(any("trailer-flip" in b for b in judge([capsule]).bad))
        self.assertTrue(judge([attest], ["capsule"]).bad, "the wrong boot's log")


if __name__ == "__main__":
    unittest.main()

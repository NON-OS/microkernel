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


"""Logs that lost lines or were cut short: a log that lost a line its pass
rests on never passes, and a capsule boot cut at its fault never does."""

import random
import unittest

from .test_fuzz import ROUNDS
from .judge import judge
from .sample import passing


class Cuts(unittest.TestCase):
    def test_losing_a_line_the_pass_rests_on_never_passes(self):
        """A line whose loss alone fails the check fails it in any company:
        losing more lines never turns a failure back into a pass."""
        rng = random.Random(2)
        base = "".join(passing()).splitlines(keepends=True)
        attack = [i for i, l in enumerate(base) if l.startswith("[ATTACK")]

        def without(gone):
            return ["".join(l for i, l in enumerate(base) if i not in gone)]

        load = [i for i in attack if judge(without({i})).bad]
        self.assertGreater(len(load), len(attack) // 2)
        for _ in range(ROUNDS):
            gone = set(rng.sample(attack, rng.randrange(1, len(attack) + 1))) | {rng.choice(load)}
            self.assertTrue(judge(without(gone)).bad, sorted(gone))

    def test_cutting_a_line_mid_way_never_crashes(self):
        rng = random.Random(3)
        attest, capsule, linux = passing()
        trap_end = capsule.index("[TRAP WALK]")
        for _ in range(ROUNDS):
            cut = rng.randrange(len(capsule))
            # Cut at or before the end of its trap, the boot shows no kernel
            # going on after the fault, so it never passes.
            if cut <= trap_end:
                self.assertTrue(judge([attest, capsule[:cut], linux]).bad, cut)
            judge([attest, capsule[cut:], linux])


if __name__ == "__main__":
    unittest.main()

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


"""The serial log is untrusted: anything a guest prints lands in it. Seeded
garbage, alone or spliced into a passing log, must never crash the check,
and garbage alone must never pass. test_fuzz_cuts.py cuts the logs."""

import random
import unittest

from .judge import judge
from .report import records
from .sample import passing
from .test_judge import with_capsule

ROUNDS = 300


def garbage(rng):
    alphabet = "[]ATCK-REFUSDP=0x19 :\n\t\x00\xff‮" + "".join(map(chr, range(32, 127)))
    return "".join(rng.choice(alphabet) for _ in range(rng.randrange(0, 2000)))


class Fuzz(unittest.TestCase):
    def test_garbage_never_crashes_and_never_passes(self):
        rng = random.Random(0x4e4f4e4f)
        for _ in range(ROUNDS):
            r = judge(garbage(rng))
            self.assertTrue(r.bad)
            list(records(r, []))

    def test_garbage_spliced_into_a_passing_log_never_crashes(self):
        rng = random.Random(1)
        base = passing()[1].splitlines(keepends=True)
        for _ in range(ROUNDS):
            lines = list(base)
            lines.insert(rng.randrange(len(lines) + 1), garbage(rng))
            judge(with_capsule(lambda t: "".join(lines)))


if __name__ == "__main__":
    unittest.main()

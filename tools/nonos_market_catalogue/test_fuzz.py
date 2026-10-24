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
"""A seeded fuzz sweep over every parser of untrusted input here: the apk
reader, the recipe checker, the index-record and name checks, the document
splitter. Each answers or refuses with ValueError or TypeError, and
nothing else may escape. NONOS_FUZZ_ITERS raises the count (default 2000)
and NONOS_FUZZ_SEED picks the seed; a failure prints both to replay. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import os
import random
import unittest
from pathlib import Path

from . import apk
from .documents import split
from .fuzz_inputs import anything, mangled
from .names import index_record, listing
from .recipes import checked

ITERS = int(os.environ.get("NONOS_FUZZ_ITERS", "2000"))
SEED = int(os.environ.get("NONOS_FUZZ_SEED", "20261024"))


class Sweep(unittest.TestCase):
    def test_every_parser_answers_or_refuses_by_name(self):
        r = random.Random(SEED)
        calls = [lambda: apk.program(mangled(r), "a"), lambda: apk.program(r.randbytes(r.randrange(300)), "a"),
                 lambda: checked(anything(r, 1), Path(".")), lambda: index_record(anything(r, 1)),
                 lambda: listing(anything(r)), lambda: split(anything(r, 1))]
        for i in range(ITERS):
            # Anything but a named refusal escapes and fails this subtest,
            # which prints the seed and iteration to replay it.
            with self.subTest(seed=SEED, iteration=i):
                try:
                    calls[i % len(calls)]()
                except (ValueError, TypeError):
                    pass


if __name__ == "__main__":
    unittest.main()

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


"""The check held to account without a boot: its unit tests, and the
mutants of passing logs, each of which must fail."""

import os
import unittest

from . import report
from .dropped import DROPPED
from .judge import judge
from .mutants import survivors
from .rules import ATTACKS, REPORT_VERSION, ROWS
from .sample import passing

HERE = os.path.dirname(os.path.abspath(__file__))


def list_rules():
    print(f"report format {report.FORMAT} v{REPORT_VERSION}")
    print("attacks:", " ".join(ATTACKS))
    for row, name, needs in ROWS:
        print(f"row {row} ({name}): {', '.join(needs)}")
    for name, (date, why) in sorted(DROPPED.items()):
        print(f"dropped {name} since {date}: {why}")
    return 0


def mutant_run(logs, what, boots=None):
    if judge(logs, boots).bad:
        print(f"attack suite mutants: {what} does not pass, so it has no mutants to judge")
        return 1
    alive, run = survivors(logs, boots)
    for desc in alive:
        print(f"SURVIVED {desc}")
    print(f"attack suite mutants: {run - len(alive)} of {run} caught over {what}")
    return 1 if alive or not run else 0


def self_test():
    tests = unittest.defaultTestLoader.discover(HERE, pattern="test_*.py", top_level_dir=os.path.dirname(HERE))
    if not unittest.TextTestRunner(verbosity=0).run(tests).wasSuccessful():
        return 1
    rc = mutant_run(passing(), "the synthetic sample logs")
    print("attack suite check: self-test " + ("PASS" if rc == 0 else "FAIL"))
    return rc

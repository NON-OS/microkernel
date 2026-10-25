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
"""One run's log, judged on its own: the workload must end well, a process
exiting 0 and none exiting otherwise or on a signal, and the log must hold
exactly one measured set, the app's own. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest

from . import verdict
from .test_verdict import OK, OTHER


class VerdictRun(unittest.TestCase):
    def test_the_workload_must_end_well_and_be_the_only_set(self):
        failed = OK.replace(b"status 0", b"status 128")  # git clone with no helper to run
        self.assertIn("workload failed", verdict.verdict("age", [failed])[2])
        signalled = OK + b"\n[LINUX] process 3 ended by signal 11"
        self.assertIn("workload failed", verdict.verdict("age", [signalled])[2])
        self.assertIn("never ended", verdict.verdict("age", [OK.replace(b"exited, status 0", b"")])[2])
        two = OK + b"\n[PROVE] syscall buckets ok=" + OTHER.encode()
        self.assertIn("2 measured sets", verdict.verdict("age", [two])[2])


if __name__ == "__main__":
    unittest.main()

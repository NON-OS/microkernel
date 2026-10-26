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
exactly one measured set, the app's own, and the store must hold its proof:
a failure after the set is a fail naming the code. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import unittest

from . import verdict
from .test_verdict import HELD, OK, OTHER


class VerdictRun(unittest.TestCase):
    def test_the_workload_must_end_well_and_be_the_only_set(self):
        failed = OK.replace(b"status 0", b"status 128")  # git clone with no helper to run
        self.assertIn("workload failed", verdict.verdict("age", [failed])[2])
        signalled = OK + b"\n[LINUX] process 3 ended by signal 11"
        self.assertIn("workload failed", verdict.verdict("age", [signalled])[2])
        self.assertIn("never ended", verdict.verdict("age", [OK.replace(b"exited, status 0", b"")])[2])
        two = OK + b"\n[PROVE] syscall buckets ok=" + OTHER.encode()
        self.assertIn("2 measured sets", verdict.verdict("age", [two])[2])

    def test_only_a_held_proof_passes_and_a_failure_after_the_set_names_its_code(self):
        unheld = OK.replace(HELD, b"")
        self.assertIn("never held", verdict.verdict("age", [unheld])[2])
        other = OK.replace(b"held: linux.age,", b"held: linux.agent,")
        self.assertIn("never held", verdict.verdict("age", [other])[2], "another app's proof")
        failed = unheld + b"\n[STORE-PROVE] failed: linux.age, -1103"
        self.assertIn("measured: -1103", verdict.verdict("age", [failed])[2])
        quiet = unheld + b"\n[STORE-PROVE] failed: linux.age, quiet"
        self.assertIn("measured: quiet", verdict.verdict("age", [quiet])[2])
        both = failed + HELD
        self.assertIn("measured: -1103", verdict.verdict("age", [both])[2], "a held line does not undo it")
        earlier = b"[STORE-PROVE] failed: linux.age, -2\n" + OK
        self.assertTrue(verdict.verdict("age", [earlier])[0], "a failure before the run is not its")


if __name__ == "__main__":
    unittest.main()

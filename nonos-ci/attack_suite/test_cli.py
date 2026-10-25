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


"""The command line's exits: 0 pass, 1 fail, 2 a file it cannot read or
write, or arguments it cannot use."""

import hashlib
import json
import os
import subprocess
import sys
import tempfile
import unittest

from .sample import passing

CLI = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "attack_suite_check.py")


def run(*argv):
    p = subprocess.run([sys.executable, CLI, *argv], capture_output=True, text=True, timeout=60)
    return p.returncode, p.stdout + p.stderr


class Cli(unittest.TestCase):
    def setUp(self):
        d = tempfile.TemporaryDirectory()
        self.addCleanup(d.cleanup)
        self.dir = d.name
        self.logs = [self.path(f"{n}.log", t) for n, t in zip(("attest", "capsule", "linux"), passing())]

    def path(self, name, body=None):
        p = os.path.join(self.dir, name)
        if body is not None:
            with open(p, "w") as f:
                f.write(body)
        return p

    def args(self, logs=None):
        return [a for p in (logs or self.logs) for a in ("--log", p)]

    def test_passing_logs_exit_0_and_the_report_names_their_bytes(self):
        js = self.path("r.json")
        rc, out = run(*self.args(), "--report", self.path("r.txt"), "--json", js)
        self.assertEqual(rc, 0, out)
        with open(js) as f:
            shas = [r["sha256"] for r in json.load(f) if r["record"] == "log"]
        with open(self.logs[1], "rb") as f:
            self.assertEqual(shas[1], hashlib.sha256(f.read()).hexdigest())

    def test_one_boot_judged_alone(self):
        self.assertEqual(run(*self.args([self.logs[1]]), "--boot", "capsule")[0], 0)
        self.assertEqual(run(*self.args([self.logs[1]]))[0], 1, "the other boots' attacks never ran")

    def test_the_mutants_hold_over_one_joined_log_and_over_three(self):
        joined = self.path("joined.log", "".join(passing()))
        self.assertEqual(run(*self.args([joined]), "--mutants")[0], 0)
        self.assertEqual(run(*self.args(), "--mutants")[0], 0)

if __name__ == "__main__":
    unittest.main()

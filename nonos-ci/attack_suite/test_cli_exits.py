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


"""The command line's failures: 1 for a verdict, 2 for what it cannot use."""

import unittest

from .test_cli import Cli, run


class Exits(Cli):
    def test_an_empty_log_exits_1(self):
        rc, out = run("--log", self.path("e.log", ""))
        self.assertEqual(rc, 1, out)
        self.assertIn("the log is empty", out)

    def test_files_it_cannot_use_exit_2(self):
        rc, out = run("--log", self.path("absent.log"))
        self.assertEqual((rc, "cannot read" in out), (2, True), out)
        rc, out = run(*self.args(), "--report", self.path("no/such/dir/r.txt"))
        self.assertEqual((rc, "cannot write" in out), (2, True), out)
        self.assertEqual(run()[0], 2)
        self.assertEqual(run("--bogus")[0], 2)
        self.assertEqual(run(*self.args(), "--boot", "nope")[0], 2)

    def test_list_and_help(self):
        rc, out = run("--list")
        self.assertEqual(rc, 0)
        self.assertIn("dropped kernel-replay since 2026-10-24", out)
        self.assertIn("--self-test", run("--help")[1])



del Cli

if __name__ == "__main__":
    unittest.main()

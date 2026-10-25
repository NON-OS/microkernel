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


"""The verdict's exit codes, and a tool that cannot start; nothing boots."""

import os
import sys
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_selftest import boots, run  # noqa: E402


class Verdict(unittest.TestCase):
    def test_the_exit_codes(self):
        self.assertEqual(run.verdict([("build", "exited 0"), ("a", "seen"), ("b", "exited 0")], 0), 0)
        self.assertEqual(run.verdict([("a", "seen")], 1), 1, "the checker failed")
        self.assertEqual(run.verdict([("a", "bound")], 0), 1, "a boot did not finish")
        self.assertEqual(run.verdict([("a", "exited 2")], 0), 1)
        self.assertEqual(run.verdict([("build", "build exited 1")], 0), 2)
        self.assertEqual(run.verdict([("a", "cannot start nix: x")], 1), 2)
        self.assertEqual(run.verdict([("a", "seen")], 2), 2, "the checker could not read a log")

    def test_a_tool_that_cannot_start_is_said_not_raised(self):
        d = tempfile.mkdtemp()
        tool = os.path.join(d, "not-executable")
        open(tool, "w").close()
        for cmd in (["/no/such/tool"], [tool]):
            b = boots.Boot("x", cmd, os.path.join(d, "x.log"), [], 0, d, {})
            self.assertTrue(run.boot(b, 5, False).startswith("cannot start"), cmd)
        self.assertTrue(run.prepare(["/no/such/tool"], 5, False).startswith("build cannot start"))


if __name__ == "__main__":
    unittest.main()

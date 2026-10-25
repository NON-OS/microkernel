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
"""The boot verdict passes an app only when its own logs show the install
in its release's box, no refusal, and a measured set; the set written back
is the union of its runs, and only passing apps' recipes change. Run:
python3 -m unittest discover -s tools/nonos_market_catalogue -t tools"""

import json
import os
import tempfile
import unittest
from pathlib import Path
from unittest import mock

from . import verdict

SET = "02" + "00" * 64  # bucket 1 only; exit and exit_group are always added
OTHER = "01" + "00" * 64
RUN = b"[LINUX-INSTALL] started, in its release's box linux.age\n[LINUX] process 2 exited, status 0\n"
OK = RUN + b"[PROVE] syscall buckets ok=" + SET.encode()


class Verdict(unittest.TestCase):
    def test_only_a_boxed_unrefused_measured_run_passes(self):
        self.assertTrue(verdict.verdict("age", [OK])[0])
        self.assertFalse(verdict.verdict("git", [OK])[0], "another app's log")
        self.assertFalse(verdict.verdict("age", [OK.replace(b", in its release's box", b"")])[0])
        self.assertFalse(verdict.verdict("age", [OK + b"\n[LINUX] refused the market's policy: x"])[0])
        self.assertFalse(verdict.verdict("age", [RUN])[0], "no set")
        bad = OK.replace(SET.encode(), ("00" * 64 + "02").encode())
        self.assertFalse(verdict.verdict("age", [bad])[0], "a bit past 512")
        self.assertFalse(verdict.verdict("age", [OK, RUN])[0], "every run must pass")

    def test_runs_unite_and_only_passing_recipes_are_written(self):
        with tempfile.TemporaryDirectory() as d:
            recipes = Path(d) / "c.json"
            recipes.write_text(json.dumps({"format": "f", "apps": [{"package": "age"}, {"package": "git"}]}))
            runs = [Path(d) / n for n in ("a1", "a2", "g")]
            runs[0].write_bytes(OK)
            runs[1].write_bytes(OK.replace(SET.encode(), OTHER.encode()))
            runs[2].write_bytes(b"nothing")
            argv = ["--write", f"age={runs[0]}", f"age={runs[1]}", f"git={runs[2]}"]
            with mock.patch.object(verdict, "RECIPES", recipes), open(os.devnull, "w") as quiet, \
                    mock.patch("sys.stdout", quiet):
                self.assertEqual(verdict.main(argv), 1)
            apps = {a["package"]: a for a in json.loads(recipes.read_text())["apps"]}
            calls = bytes.fromhex(apps["age"]["calls"])
            one_run = (calls[0] & 1, calls[0] >> 1 & 1)  # bucket 0 from a2, bucket 1 from a1
            self.assertEqual(one_run + (calls[7] >> 4 & 1, calls[28] >> 7 & 1), (1, 1, 1, 1))
            self.assertNotIn("calls", apps["git"])

    def test_no_pairs_is_usage(self):
        with open(os.devnull, "w") as quiet, mock.patch("sys.stderr", quiet):
            self.assertEqual(verdict.main([]), 2)
            self.assertEqual(verdict.main(["age"]), 2)


if __name__ == "__main__":
    unittest.main()

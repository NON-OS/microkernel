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

"""A step that times out leaves the machine's state behind, every CPU's code
read from that CPU, and a monitor that has gone costs the run nothing more."""

import sys
import tempfile
import unittest
import unittest.mock
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "userland/capsule_install/e2e"))

from nonos_drive import dump, step  # noqa: E402


class Machine:
    """Answers QMP as a stopped four-CPU guest would; `gone` drops the line.
    As QEMU does, it forgets a `cpu N` line at the end of its call."""

    def __init__(self, gone=False):
        self.gone = gone

    def cmd(self, name, **args):
        if self.gone:
            raise ConnectionError("QMP closed")
        if name == "query-cpus-fast":
            return {"return": [{"cpu-index": i} for i in range(4)]}
        return {"return": f"{args['command-line']} on cpu {args.get('cpu-index', 0)}"}


class Stuck(unittest.TestCase):
    def test_every_cpu_shows_its_own_code(self):
        with tempfile.TemporaryDirectory() as d:
            text = dump.dump(Machine(), Path(d)).read_text()
        self.assertIn("info registers -a on cpu 0", text)
        for cpu in range(4):
            self.assertIn(f"== cpu {cpu}: x/16i $pc\nx/16i $pc on cpu {cpu}", text)

    def test_a_monitor_that_has_gone_writes_nothing(self):
        with tempfile.TemporaryDirectory() as d:
            self.assertIsNone(dump.dump(Machine(gone=True), Path(d)))
            self.assertFalse((Path(d) / dump.FILE).exists())

    def test_only_a_step_that_timed_out_leaves_it(self):
        for seen in (True, False):
            with tempfile.TemporaryDirectory() as d, unittest.mock.patch.object(
                    step, "until", return_value=seen):
                self.assertEqual(step.run(Machine(), "until:x:1", Path(d), "s"), seen)
                self.assertEqual((Path(d) / dump.FILE).exists(), not seen)


if __name__ == "__main__":
    unittest.main()

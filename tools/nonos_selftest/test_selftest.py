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


"""The bounded boot, with stand-in commands; nothing here boots. Run
with python3 -m unittest discover -s tools/nonos_selftest -p 'test_*.py'."""

import os
import sys
import tempfile
import time
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_selftest import watch  # noqa: E402

watch.POLL = 0.05


def alive(pid):
    """Running, not gone and not a zombie no parent has reaped yet."""
    try:
        with open(f"/proc/{pid}/stat") as f:
            return f.read().rsplit(")", 1)[1].split()[0] != "Z"
    except FileNotFoundError:
        return False


def script(body):
    return [sys.executable, "-c", body]


class Watch(unittest.TestCase):
    def setUp(self):
        d = tempfile.TemporaryDirectory()
        self.addCleanup(d.cleanup)
        self.log, self.pid = os.path.join(d.name, "boot.log"), os.path.join(d.name, "child.pid")

    def test_seen_then_settled_and_the_whole_group_stopped(self):
        # The command starts a child of its own, as make starts QEMU; both go.
        body = (f"import subprocess,sys,time; c=subprocess.Popen([sys.executable,'-c','import time; time.sleep(60)']);"
                f"open({self.pid!r},'w').write(str(c.pid)); open({self.log!r},'w').write('[ATTACK-FAULT] pid 4\\n');"
                "time.sleep(60)")
        t = time.monotonic()
        self.assertEqual(watch.run(script(body), self.log, [r"\[ATTACK-FAULT\]"], 0.3, 30), "seen")
        self.assertLess(time.monotonic() - t, 20)
        child = int(open(self.pid).read())
        time.sleep(0.2)
        self.assertFalse(alive(child), "the boot's child outlived it")

    def test_the_bound_ends_a_boot_that_never_shows_its_lines(self):
        self.assertEqual(watch.run(script("import time; time.sleep(60)"), self.log, ["never"], 0, 0.5), "bound")

    def test_a_command_that_ends_says_how(self):
        self.assertEqual(watch.run(script("raise SystemExit(3)"), self.log, [], 0, 10), "exited 3")
        self.assertEqual(watch.run(script("pass"), self.log, ["x"], 0, 10), "exited 0")


if __name__ == "__main__":
    unittest.main()

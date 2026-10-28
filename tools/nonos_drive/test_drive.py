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
read from that CPU, and a monitor that has gone costs the run nothing more.
A say: step captions a recording and only the log of a drive not filmed;
pause: is wait: by another name."""

import contextlib
import io
import sys
import tempfile
import threading
import time
import unittest
import unittest.mock
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "userland/capsule_install/e2e"))

from nonos_drive import dump, record, step  # noqa: E402


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

    def test_a_regs_step_names_its_file_and_the_run_goes_on(self):
        with tempfile.TemporaryDirectory() as d:
            self.assertTrue(step.run(Machine(), "regs:kpx1", Path(d), Path(d) / "serial.log"))
            self.assertIn("info registers -a", (Path(d) / "regs-kpx1.txt").read_text())
            self.assertFalse((Path(d) / dump.FILE).exists())

    def test_only_a_step_that_timed_out_leaves_it(self):
        for seen in (True, False):
            with tempfile.TemporaryDirectory() as d, unittest.mock.patch.object(
                    step, "until", return_value=seen):
                self.assertEqual(step.run(Machine(), "until:x:1", Path(d), "s"), seen)
                self.assertEqual((Path(d) / dump.FILE).exists(), not seen)


class Untouched:
    """A machine no step should reach: any QMP call fails the test."""

    def cmd(self, name, **args):
        raise AssertionError(f"a say or pause step sent {name}")


def said(s, film=None, serial_path="s"):
    out = io.StringIO()
    with contextlib.redirect_stdout(out), tempfile.TemporaryDirectory() as d:
        ok = step.run(Untouched(), s, Path(d), str(serial_path), film)
    return ok, out.getvalue()


def filmed(d, started_ago=0.0):
    """A Film whose processes no step touches, its caption in `d`."""
    return record.Film(99, Path(d) / "caption.txt", Path(d) / "t.mp4", None, None,
                       started=time.time() - started_ago)


class Say(unittest.TestCase):
    def test_a_drive_not_filmed_only_logs_it(self):
        with unittest.mock.patch.object(step, "say") as write:
            ok, log = said("say:The boot menu: Standard, Hardened, Air-Gapped.")
        self.assertTrue(ok)
        write.assert_not_called()
        self.assertIn("say 'The boot menu: Standard, Hardened, Air-Gapped.'", log)

    def test_a_recording_gets_the_whole_text_colons_and_all(self):
        with tempfile.TemporaryDirectory() as d:
            film = filmed(d)
            self.assertTrue(said("say:Mode: amnesic, by default.", film)[0])
            self.assertEqual(film.caption.read_text(), "Mode: amnesic, by default.")

    def test_say_alone_clears_it(self):
        with tempfile.TemporaryDirectory() as d:
            film = filmed(d)
            said("say:on screen", film)
            ok, log = said("say:", film)
            self.assertTrue(ok)
            self.assertEqual(film.caption.read_text(), "")
        self.assertIn("caption cleared", log)

    def test_each_caption_is_noted_at_its_time_in_the_video(self):
        with tempfile.TemporaryDirectory() as d:
            film = filmed(d, started_ago=3725.0)
            log = said("say:The desktop.", film)[1]
            cues = (Path(d) / record.CUES).read_text()
        self.assertTrue(cues.startswith("1:02:05."), cues)
        self.assertTrue(cues.endswith("\tThe desktop.\n"), cues)
        self.assertIn("say 'The desktop.' at 1:02:05.", log)

    def test_pause_sleeps_as_wait_does(self):
        for s in ("pause:2.5", "wait:2.5"):
            with unittest.mock.patch.object(step.time, "sleep") as sleep:
                self.assertTrue(said(s)[0])
            sleep.assert_called_once_with(2.5)

    def test_after_waits_for_a_second_install_not_the_first(self):
        with tempfile.TemporaryDirectory() as d:
            serial = Path(d) / "serial.log"
            serial.write_text("[LINUX] installed\n")
            with contextlib.redirect_stdout(io.StringIO()):
                self.assertFalse(step.run(Machine(gone=True), "after:[LINUX] installed:1", Path(d),
                                          str(serial)), "the first install's line does not count")

            def second():
                with serial.open("a") as f:
                    f.write("[LINUX] installed\n")
            threading.Timer(0.3, second).start()
            self.assertTrue(said("after:[LINUX] installed:5", serial_path=serial)[0])

    def test_an_unknown_step_still_stops_the_run(self):
        with self.assertRaises(SystemExit):
            said("sya:typo")


if __name__ == "__main__":
    unittest.main()

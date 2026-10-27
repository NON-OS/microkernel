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

"""A drive filmed: the X display and the ffmpeg line it is filmed with, a cut
that leaves a whole file, and the order things stop in: ffmpeg, then QEMU,
then Xvfb, whether the steps finished or one raised. A drive not filmed
starts QEMU headless, as it always has. Nothing here starts QEMU, Xvfb or
ffmpeg."""

import importlib.machinery
import importlib.util
import sys
import tempfile
import time
import unittest
import unittest.mock
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "userland/capsule_install/e2e"))

from nonos_drive import fill, machine, record  # noqa: E402


def value(argv, flag):
    return argv[argv.index(flag) + 1]


class Commands(unittest.TestCase):
    def test_xvfb_is_one_1080p_screen_with_no_tcp(self):
        self.assertEqual(record.xvfb_cmd(99),
                         ["Xvfb", ":99", "-screen", "0", "1920x1080x24", "-nolisten", "tcp"])

    def test_ffmpeg_films_the_display_whole_with_no_pointer(self):
        argv = record.ffmpeg_cmd(101, Path("/r/font.ttf"), Path("/o/caption.txt"), Path("/o/t.mp4"))
        self.assertEqual(argv[0], "ffmpeg")
        self.assertEqual(value(argv, "-f"), "x11grab")
        self.assertEqual(value(argv, "-video_size"), "1920x1080")
        self.assertEqual(value(argv, "-framerate"), "30")
        self.assertEqual(value(argv, "-draw_mouse"), "0")
        self.assertEqual(value(argv, "-i"), ":101")
        self.assertLess(argv.index("-f"), argv.index("-i"), "x11grab is the input's format")

    def test_the_encoding_plays_from_the_start(self):
        argv = record.ffmpeg_cmd(99, Path("/f.ttf"), Path("/c.txt"), Path("/o/t.mp4"))
        want = {"-c:v": "libx264", "-preset": "veryfast", "-crf": "20", "-pix_fmt": "yuv420p",
                "-movflags": "+faststart"}
        self.assertEqual({k: value(argv, k) for k in want}, want)
        self.assertEqual(argv[-1], "/o/t.mp4")

    def test_the_caption_is_read_again_every_frame_as_written(self):
        vf = value(record.ffmpeg_cmd(99, Path("/r/Geist-SemiBold.ttf"), Path("/o/caption.txt"),
                                     Path("/o/t.mp4")), "-vf")
        self.assertTrue(vf.startswith("drawtext="), vf)
        opts = dict(o.split("=", 1) for o in vf[len("drawtext="):].split(":"))
        self.assertEqual(opts["fontfile"], "'/r/Geist-SemiBold.ttf'")
        self.assertEqual(opts["textfile"], "'/o/caption.txt'")
        self.assertEqual((opts["reload"], opts["expansion"]), ("1", "none"))
        self.assertEqual((opts["fontcolor"], opts["box"]), ("white", "1"))
        self.assertTrue(opts["boxcolor"].startswith("black@0."), "a translucent dark box")
        self.assertEqual(opts["x"], "(w-text_w)/2", "centred")
        self.assertGreaterEqual(int(opts["fontsize"]), 36, "readable at 1080p")

    def test_the_brand_font_is_in_the_tree(self):
        self.assertTrue((ROOT / record.FONT).is_file(), record.FONT)
        self.assertTrue(record.FONT.endswith("Geist-SemiBold.ttf"))

    def test_a_path_with_a_quote_is_refused(self):
        with self.assertRaises(ValueError):
            record.drawtext("/f.ttf", "/o/it's.txt")

    def test_the_window_opens_on_the_display_over_x(self):
        env = record.display_env(7, {"WAYLAND_DISPLAY": "wayland-0", "DISPLAY": ":0", "HOME": "/h"})
        self.assertEqual((env["DISPLAY"], env["GDK_BACKEND"], env["HOME"]), (":7", "x11", "/h"))
        self.assertNotIn("WAYLAND_DISPLAY", env)

    def test_only_a_mapped_window_off_the_screen_is_moved(self):
        found = [(1, fill.IS_VIEWABLE, 0, 0, 640, 480),     # GTK's first size, in the corner
                 (2, fill.IS_VIEWABLE, 0, 0, 1920, 1080),   # already full screen
                 (3, 0, -100, -100, 1, 1)]                  # GDK's unmapped leader window
        self.assertEqual(fill.misplaced(found, 1920, 1080), [1])

    def test_a_drive_not_filmed_is_headless_as_before(self):
        argv = machine.command("i.img", "o/serial.log", ["--lane", "usb-tablet"])
        self.assertEqual(argv, ["nix", "run", ".#qemu", "--", "--image", "i.img", "--tpm",
                                "--headless", "--serial", "o/serial.log", "--lane", "usb-tablet"])
        shown = machine.command("i.img", "o/serial.log", [], shown=True)
        self.assertIn("--shown", shown)
        self.assertNotIn("--headless", shown)


class Stdin:
    def __init__(self, log):
        self.log, self.sent, self.closed = log, b"", False

    def write(self, data):
        self.sent += data
        self.log.append(("ffmpeg stdin", data))

    def flush(self):
        pass

    def close(self):
        self.closed = True


class Proc:
    """A process that leaves when told: ffmpeg on `q`, the rest on SIGTERM,
    each noted in `log` as it goes."""

    def __init__(self, name, log, status=0):
        self.name, self.log, self.status, self.returncode = name, log, status, None
        self.stdin = Stdin(log)

    def poll(self):
        return self.returncode

    def wait(self, timeout=None):
        if self.returncode is None:
            self.returncode = self.status
            self.log.append(("stopped", self.name))
        return self.returncode

    def terminate(self):
        self.log.append(("terminate", self.name))

    def kill(self):
        self.log.append(("kill", self.name))


def film(log, status=0, text=Path("/o/caption.txt")):
    return record.Film(99, text, Path("/o/t.mp4"), Proc("Xvfb", log), Proc("ffmpeg", log, status),
                       started=time.time())


class Cut(unittest.TestCase):
    def test_a_cut_is_q_then_a_wait_and_says_whether_the_file_is_whole(self):
        log = []
        f = film(log)
        self.assertTrue(record.cut(f))
        self.assertEqual(f.ffmpeg.stdin.sent, b"q")
        self.assertTrue(f.ffmpeg.stdin.closed)
        self.assertEqual(log, [("ffmpeg stdin", b"q"), ("stopped", "ffmpeg")])
        self.assertNotIn(("terminate", "ffmpeg"), log, "q, never a signal, when it leaves")

    def test_a_second_cut_changes_nothing(self):
        log = []
        f = film(log)
        record.cut(f)
        before = list(log)
        self.assertTrue(record.cut(f))
        self.assertEqual(log, before)

    def test_ffmpeg_that_failed_is_a_video_not_whole(self):
        self.assertFalse(record.cut(film([], status=1)))

    def test_no_film_is_nothing_to_cut(self):
        self.assertTrue(record.cut(None))


def drive_script():
    """tools/nonos-drive as a module, to run its drive() with the machine,
    QMP and the filming stood in for."""
    path = str(ROOT / "tools/nonos-drive")
    loader = importlib.machinery.SourceFileLoader("nonos_drive_main", path)
    spec = importlib.util.spec_from_loader(loader.name, loader)
    module = importlib.util.module_from_spec(spec)
    loader.exec_module(module)
    return module


class Machine:
    def __init__(self, log, raises=False):
        self.log, self.raises = log, raises

    def cmd(self, name, **args):
        if self.raises:
            raise RuntimeError("QEMU refused it")
        return {"return": {}}

    def quit(self):
        self.log.append(("qmp", "quit"))


class Order(unittest.TestCase):
    def run_drive(self, steps, raises=False, recorded=True):
        main, log = drive_script(), []
        qemu = Proc("qemu", log)
        args = unittest.mock.Mock(image="i.img", qemu_arg=[], record=Path("/o/t.mp4") if recorded else None)
        started = []

        def start(root, image, out, serial, qemu_args, display=None):
            started.append(display)
            return qemu

        def films(root, out, workdir):
            log.append(("started", "Xvfb and ffmpeg"))
            return film(log, text=Path(workdir) / "caption.txt")

        with tempfile.TemporaryDirectory() as d, \
                unittest.mock.patch.object(main.machine, "start", start), \
                unittest.mock.patch.object(main.machine, "stop", lambda p: p.wait()), \
                unittest.mock.patch.object(main.record, "start", films), \
                unittest.mock.patch.object(main, "Qmp", lambda path, wait: Machine(log, raises)):
            args.out = Path(d)
            try:
                with main.record.filming(ROOT, args.record, args.out) as f:
                    ok = main.drive(args, steps, args.out / "serial.log", f)
            except RuntimeError:
                ok = None
        return ok, log, started

    def stops(self, log):
        return [name for what, name in log if what == "stopped"]

    def test_ffmpeg_then_qemu_then_xvfb(self):
        ok, log, started = self.run_drive(["say:hello", "pause:0", "say:"])
        self.assertTrue(ok)
        self.assertEqual(started, [99], "QEMU's window on the film's display")
        self.assertEqual(self.stops(log), ["ffmpeg", "qemu", "Xvfb"])
        self.assertLess(log.index(("ffmpeg stdin", b"q")), log.index(("qmp", "quit")))

    def test_a_step_that_raises_still_leaves_a_whole_video(self):
        ok, log, _ = self.run_drive(["click"], raises=True)
        self.assertIsNone(ok, "the step's error reaches the caller")
        self.assertEqual(self.stops(log), ["ffmpeg", "qemu", "Xvfb"])
        self.assertIn(("ffmpeg stdin", b"q"), log)

    def test_a_drive_not_filmed_starts_no_display(self):
        ok, log, started = self.run_drive(["pause:0"], recorded=False)
        self.assertTrue(ok)
        self.assertEqual(started, [None], "headless, as before")
        self.assertEqual(self.stops(log), ["qemu"])


TOUR = ROOT / "tools/nonos_tutorial/tour.steps"
# The QKeyCode names (QEMU's qapi/ui.json) a key: step in the tour may use.
QCODES = {*"abcdefghijklmnopqrstuvwxyz0123456789", "ret", "esc", "tab", "spc", "up", "down",
          "left", "right", "home", "end", "pgup", "pgdn", "ctrl", "alt", "shift", "backspace",
          "delete", "equal", "minus", "power", *(f"f{n}" for n in range(1, 13))}
# One line across the screen at the caption's size, with room to spare.
CAPTION_MAX = 80


def tour_steps():
    lines = (line.strip() for line in TOUR.read_text().splitlines())
    return [line for line in lines if line and not line.startswith("#")]


class Tour(unittest.TestCase):
    """The tutorial is an hour on a KVM host; a step the drive does not know,
    or a character it cannot type, would stop it at that step, so each is
    run here against a machine that takes every command."""

    def test_every_step_is_one_the_drive_takes(self):
        from qmp import Qmp
        from nonos_drive import step
        q = Qmp.__new__(Qmp)
        q.cmd = lambda name, **args: {"return": {}}
        with tempfile.TemporaryDirectory() as d, \
                unittest.mock.patch.object(step, "until", return_value=True), \
                unittest.mock.patch.object(step, "after", return_value=True), \
                unittest.mock.patch("time.sleep"), \
                unittest.mock.patch("sys.stdout"):
            serial = Path(d) / "serial.log"
            serial.write_text("")
            for s in tour_steps():
                with self.subTest(step=s):
                    self.assertTrue(step.run(q, s, Path(d), str(serial), film(log=[], text=Path(d) / "c.txt")))

    def test_every_key_is_a_qemu_key(self):
        for s in tour_steps():
            if s.startswith("key:"):
                self.assertLessEqual(set(s[4:].split("+")), QCODES, s)

    def test_every_caption_fits_one_line_in_the_font(self):
        for s in tour_steps():
            if s.startswith("say:"):
                text = s[4:]
                self.assertLessEqual(len(text), CAPTION_MAX, text)
                self.assertTrue(all(c.isascii() and c.isprintable() or c == "Ø" for c in text), text)

    def test_every_optional_section_is_closed(self):
        """Each block closes, the innermost first, so sed's range from a
        block's opening line to its closing one is that block and no more."""
        open_, seen = [], set()
        for line in TOUR.read_text().splitlines():
            if line.startswith("# >>> "):
                self.assertNotIn(line.split()[2], open_, line)
                open_.append(line.split()[2])
            elif line.startswith("# <<< "):
                self.assertEqual(line.split()[2], open_[-1] if open_ else None, line)
                seen.add(open_.pop())
        self.assertEqual(open_, [], "a section is never closed")
        self.assertLessEqual({"market", "simplex", "telegram", "monero", "qwen", "dev-image",
                              "sealed"}, seen)


if __name__ == "__main__":
    unittest.main()

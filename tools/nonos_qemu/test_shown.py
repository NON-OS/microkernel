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

"""--shown is the window a recording films: GTK, full screen, no menu bar,
and the serial console to the --serial file, as --headless sends it, so a
drive reads it the same way. --headless and the window to use are as they
were. Nothing here starts QEMU."""

import contextlib
import io
import os
import sys
import unittest
import unittest.mock

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import __main__ as launcher, machine  # noqa: E402


def parsed(*argv):
    with unittest.mock.patch.object(sys, "argv", ["nonos-qemu", *argv]):
        return launcher.args()


class Shown(unittest.TestCase):
    def test_shown_is_full_screen_gtk_with_no_menu_bar(self):
        argv = machine.display(False, shown=True)
        self.assertEqual(argv[argv.index("-display") + 1],
                         "gtk,zoom-to-fit=on,full-screen=on,show-menubar=off")
        self.assertEqual(argv[argv.index("-vga") + 1], "none", "the one display, as the window has")

    def test_headless_and_the_window_are_as_they_were(self):
        self.assertEqual(machine.display(True), ["-display", "none"])
        self.assertEqual(machine.display(True, shown=False), ["-display", "none"])
        window = machine.display(False)
        self.assertIn("zoom-to-fit=on", window[-1])
        self.assertNotIn("full-screen", window[-1])

    def test_shown_sends_the_serial_to_the_file(self):
        a = parsed("--shown", "--serial", "out/serial.log")
        self.assertTrue(a.shown)
        self.assertFalse(a.headless)
        self.assertEqual(machine.serial(a.serial, a.headless or a.shown),
                         ["-serial", "file:out/serial.log"])

    def test_the_window_to_use_keeps_the_monitor_on_stdio(self):
        a = parsed()
        self.assertEqual(machine.serial(a.serial, a.headless or a.shown), ["-serial", "mon:stdio"])

    def test_shown_and_headless_together_are_refused(self):
        with contextlib.redirect_stderr(io.StringIO()) as said, self.assertRaises(SystemExit):
            parsed("--shown", "--headless")
        self.assertIn("not allowed with", said.getvalue())


if __name__ == "__main__":
    unittest.main()

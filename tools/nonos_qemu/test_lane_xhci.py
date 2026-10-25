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

"""The USB devices on the machine's own xHCI controller. Nothing here starts
QEMU."""

import os
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import lanes  # noqa: E402


class XhciLanes(unittest.TestCase):
    def test_the_keyboard_and_the_stick_are_on_the_xhci_controller(self):
        p = lanes.plan(["usb-kbd", "usb-tablet", "xhci-stick"], "/d")
        args = " ".join(p.args)
        for device in ("usb-kbd,bus=xhci.0", "usb-tablet,bus=xhci.0",
                       "usb-storage,bus=xhci.0,drive=lane-xhci-stick-disk,removable=on"):
            self.assertIn(f"-device {device}", args)
        self.assertEqual(p.images, [("/d/lane-xhci-stick.img", 256 << 20)])
        self.assertEqual(p.swaps, {})


if __name__ == "__main__":
    unittest.main()

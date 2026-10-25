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

"""The display lanes that take the virtio GPU's place. Nothing here starts
QEMU."""

import os
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import lanes, machine  # noqa: E402


class GpuLanes(unittest.TestCase):
    def test_each_display_lane_is_the_one_display(self):
        plain = machine.devices("nat", "/s", None)
        for lane, model in (("gpu=bga", "VGA"), ("gpu=gop-only", "ramfb"), ("vmware-svga", "vmware-svga")):
            swapped = machine.devices("nat", "/s", None, lanes.plan([lane], "/d").swaps)
            self.assertEqual([a for a in swapped if a not in plain], [model], lane)
            self.assertFalse(any(a.startswith("virtio-vga") for a in swapped), lane)
        with self.assertRaises(lanes.LaneError):
            lanes.plan(["gpu=bga", "gpu=gop-only"], "/d")
        with self.assertRaises(lanes.LaneError):
            lanes.plan(["gpu=bga", "vmware-svga"], "/d")

    def test_a_sized_virtio_lane_is_the_virtio_gpu_at_its_size(self):
        plain = machine.devices("nat", "/s", None)
        swapped = machine.devices("nat", "/s", None, lanes.plan(["gpu=virtio-1280x800"], "/d").swaps)
        self.assertEqual([a for a in swapped if a not in plain],
                         ["virtio-vga,vectors=0,edid=on,xres=1280,yres=800"])
        self.assertEqual(len(swapped), len(plain))
        with self.assertRaises(lanes.LaneError):
            lanes.plan(["gpu=virtio-1280x800", "gpu=bga"], "/d")
        with self.assertRaises(lanes.LaneError):
            lanes.plan(["gpu=virtio-1280x801"], "/d")


if __name__ == "__main__":
    unittest.main()

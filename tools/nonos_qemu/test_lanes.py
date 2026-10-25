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

"""The lanes' lines; nothing here starts QEMU. Run them with
python3 -m unittest discover -s tools/nonos_qemu -p 'test_*.py'."""

import os
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import lanes  # noqa: E402
from nonos_qemu.lane_disks import DISKS  # noqa: E402

ALL = [n for n in lanes.NAMES if not n.startswith(("nic=", "iommu=", "gpu=")) and n != "bochs-display"]
ALL += ["nic=igb", "iommu=intel"]


def ids(args):
    """Every id= and every drive= the lines name."""
    pairs = [p for line in args for p in line.split(",")]
    return [p[3:] for p in pairs if p.startswith("id=")], [p[6:] for p in pairs if p.startswith("drive=")]


class Lanes(unittest.TestCase):
    def test_every_lane_together_is_one_machine(self):
        p = lanes.plan(ALL, "/d")
        made, used = ids(p.args)
        self.assertEqual(len(made), len(set(made)), "an id given twice")
        self.assertTrue(set(used) <= set(made), "a drive= with no -drive")
        self.assertTrue(all(i.startswith("lane-") for i in made))
        self.assertEqual(p.swaps, {"nic": "igb", "tpm": "tpm-tis", "gpu": "vmware-svga",
                                   "iommu": "intel", "audio": "beside-nic"})

    def test_a_lanes_disk_is_its_own_and_a_power_of_two(self):
        p = lanes.plan([*DISKS, "emmc"], "/a dir")
        self.assertEqual(len(p.images), len(DISKS), "a lane named twice is one lane")
        for path, size in p.images:
            self.assertEqual(os.path.dirname(path), "/a dir")
            self.assertEqual(size & (size - 1), 0, path)
            self.assertTrue(any(w.startswith(f"file={path},") for w in p.args), "a path is one argument")

    def test_one_nic_a_known_one(self):
        for bad in (["nic=e1000", "nic=igb"], ["nic=virtio-net-pci"], ["emmc", "sdcard"]):
            with self.assertRaises(lanes.LaneError, msg=bad):
                lanes.plan(bad, "/d")
        self.assertEqual(lanes.plan(["nic=e1000", "nic=e1000"], "/d").swaps, {"nic": "e1000"})


if __name__ == "__main__":
    unittest.main()

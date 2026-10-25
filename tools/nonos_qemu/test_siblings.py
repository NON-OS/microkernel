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

"""The hda-beside-nic lane's command line; nothing here starts QEMU."""

import os
import subprocess
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import iommu, lanes, siblings  # noqa: E402
from nonos_qemu.test_iommu import base, values  # noqa: E402

PAIR = ["hda-beside-nic", "nic=e1000"]


class Siblings(unittest.TestCase):
    def test_no_lane_leaves_the_machine_alone(self):
        self.assertEqual(siblings.apply(base(), {}), base())

    def test_the_hda_and_the_nic_are_two_functions_behind_one_root_port(self):
        swaps = lanes.plan(PAIR, "/d").swaps
        devices = values(siblings.apply(base(swaps), swaps), "-device")
        self.assertEqual(devices[0], siblings.PORT)
        self.assertIn("intel-hda,bus=lane-siblings,addr=0.0,multifunction=on", devices)
        self.assertIn("e1000,netdev=net0,bus=lane-siblings,addr=0.1", devices)
        self.assertEqual(len(devices), len(values(base(swaps), "-device")) + 1)

    def test_the_iommu_still_comes_before_the_root_port(self):
        swaps = lanes.plan([*PAIR, "iommu=intel"], "/d").swaps
        devices = values(iommu.apply(siblings.apply(base(swaps), swaps), "intel"), "-device")
        self.assertTrue(devices[0].startswith("intel-iommu,"), devices[0])
        self.assertEqual(devices[1], siblings.PORT)

    def test_it_needs_an_intel_nic_and_the_machine_to_have_both(self):
        with self.assertRaises(lanes.LaneError):
            lanes.plan(["hda-beside-nic"], "/d")
        swaps = lanes.plan(PAIR, "/d").swaps
        without_nic = [w for w in base(swaps) if "netdev=net0" not in w]
        with self.assertRaises(ValueError):
            siblings.apply(without_nic, swaps)
        with self.assertRaises(ValueError):
            siblings.apply(base(swaps), {"audio": "elsewhere"})

    def test_make_refuses_it_rather_than_boot_the_pair_apart(self):
        tool = os.path.join(os.path.dirname(HERE), "nonos-qemu-lanes")
        r = subprocess.run([sys.executable, tool, "args", "--dir", "/d", *PAIR],
                           capture_output=True, text=True)
        self.assertNotEqual(r.returncode, 0)
        self.assertIn("nix run .#qemu -- --lane hda-beside-nic", r.stderr)


if __name__ == "__main__":
    unittest.main()

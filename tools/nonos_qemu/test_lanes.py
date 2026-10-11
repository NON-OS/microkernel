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

"""The device lanes: python3 -m unittest discover -s tools/nonos_qemu -p 'test_*.py'.
Nothing here starts QEMU; it checks the lines a lane hands it."""

import os
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import lanes  # noqa: E402

ALL = [n for n in lanes.NAMES if not n.startswith("nic=")] + ["nic=igb"]


def ids(args):
    """Every id= and every drive= a device line names."""
    pairs = [p for line in args for p in line.split(",")]
    return [p.split("=", 1)[1] for p in pairs if p.startswith("id=")], \
        [p.split("=", 1)[1] for p in pairs if p.startswith("drive=")]


class Lanes(unittest.TestCase):
    def test_every_lane_together_is_one_machine(self):
        p = lanes.plan(ALL, "/d")
        made, used = ids(p.args)
        self.assertEqual(len(made), len(set(made)), "an id given twice")
        self.assertTrue(set(used) <= set(made), "a drive= with no -drive")
        self.assertTrue(all(i.startswith("lane-") for i in made))
        self.assertEqual((p.nic, p.tpm), ("igb", "tpm-tis"))

    def test_a_lanes_disk_is_its_own_and_a_card_a_power_of_two(self):
        p = lanes.plan(["emmc", "sd", "ufs", "ehci", "uhci", "ohci", "emmc"], "/d")
        self.assertEqual(len(p.images), 6, "a lane named twice is one lane")
        for path, size in p.images:
            self.assertEqual(os.path.dirname(path), "/d")
            self.assertEqual(size & (size - 1), 0, path)
            self.assertIn(f"file={path},", " ".join(p.args))

    def test_the_hosts_are_the_ones_asked_for(self):
        args = " ".join(lanes.plan(["emmc", "sd", "ufs"], "/d").args)
        self.assertIn("sdhci-pci,capareg=0x457834b4 ", args)
        self.assertIn("-device sdhci-pci -drive", args, "the card slot keeps the default word")
        self.assertIn("ufs-lu,drive=lane-ufs-lu0,lun=0,bus=lane-ufs", args)
        for host, bus in (("ehci", "lane-ehci.0"), ("uhci", "lane-uhci.0"), ("ohci", "lane-ohci.0")):
            usb = " ".join(lanes.plan([host], "/d").args)
            self.assertIn(f"usb-kbd,bus={bus},port=1", usb)
            self.assertIn(f"usb-storage,bus={bus},port=2", usb)
        self.assertIn("masterbus=lane-ehci.0", " ".join(lanes.plan(["ehci"], "/d").args))

    def test_one_nic_a_known_one(self):
        for bad in (["nic=e1000", "nic=igb"], ["nic=virtio-net-pci"], ["emmc", "sdcard"]):
            with self.assertRaises(lanes.LaneError, msg=bad):
                lanes.plan(bad, "/d")
        self.assertEqual(lanes.plan(["nic=e1000", "nic=e1000"], "/d").nic, "e1000")


if __name__ == "__main__":
    unittest.main()

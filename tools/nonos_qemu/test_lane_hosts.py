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

"""The hosts each lane asks QEMU for, on the buses its devices name."""

import os
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import lanes, machine  # noqa: E402


class LaneHosts(unittest.TestCase):
    def test_the_hosts_are_the_ones_asked_for(self):
        args = " ".join(lanes.plan(["emmc", "sd", "ufs"], "/d").args)
        self.assertIn("sdhci-pci,capareg=0x457834b4 ", args)
        self.assertIn("-device sdhci-pci -drive", args, "the card slot keeps the default word")
        self.assertIn("ufs-lu,drive=lane-ufs-disk,lun=0,bus=lane-ufs", args)
        scsi = " ".join(lanes.plan(["pvscsi", "piix4-ide"], "/d").args)
        self.assertIn("-device pvscsi,id=lane-pvscsi -drive", scsi)
        self.assertIn("scsi-hd,bus=lane-pvscsi.0,drive=lane-pvscsi-disk", scsi)
        self.assertIn("ide-hd,bus=lane-ide.0,drive=lane-piix4-ide-disk", scsi)
        for host, bus in (("ehci", "lane-ehci.0"), ("uhci", "lane-uhci.0"), ("ohci", "lane-ohci.0")):
            usb = " ".join(lanes.plan([host], "/d").args)
            self.assertIn(f"usb-kbd,bus={bus},port=1", usb)
            self.assertIn(f"usb-storage,bus={bus},port=2", usb)
        self.assertIn("masterbus=lane-ehci.0", " ".join(lanes.plan(["ehci"], "/d").args))


    def test_track_ds_devices_are_each_a_lane(self):
        named = ["pvscsi", "lsi53c895a", "mptsas1068", "AC97", "vmware-svga", "usb-tablet", "virtio-scsi",
                 "virtio-tablet", "virtio-kbd", "virtio-balloon", "piix4-ide", "vmbus"]
        p = lanes.plan(named, "/d")
        args = " ".join(p.args)
        for device in ("lsi53c895a,", "mptsas1068,", "AC97,", "usb-tablet,", "virtio-scsi-pci,",
                       "virtio-tablet-pci", "virtio-keyboard-pci", "virtio-balloon-pci", "piix4-ide,",
                       "vmbus-bridge", "hv-balloon"):
            self.assertIn(f"-device {device}", args)
        self.assertEqual(p.swaps, {"gpu": "vmware-svga"})
        for nic in ("pcnet", "vmxnet3", "tulip"):
            self.assertEqual(lanes.plan([f"nic={nic}"], "/d").swaps, {"nic": nic})

    def test_the_usb_net_bochs_and_ccid_lanes(self):
        p = lanes.plan(["nic=usb-net", "bochs-display", "usb-ccid"], "/d")
        self.assertEqual(p.swaps, {"nic": "usb-net,bus=xhci.0", "gpu": "bochs-display"})
        self.assertIn("-device usb-ccid,bus=xhci.0", " ".join(p.args))
        args = machine.devices("nat", None, None, p.swaps)
        self.assertIn("usb-net,bus=xhci.0,netdev=net0", args)
        self.assertEqual(args[1], "bochs-display")
        self.assertEqual(lanes.plan(["gpu=bga"], "/d").swaps, {"gpu": "VGA"})

if __name__ == "__main__":
    unittest.main()

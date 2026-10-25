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

"""The lanes against the rest of the machine, and against make. Nothing
here starts QEMU."""

import os
import subprocess
import sys
import tempfile
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import lanes, machine  # noqa: E402

# The whole-machine lanes make refuses (test_siblings.py) are left out.
ALL = [n for n in lanes.NAMES if not n.startswith(("nic=", "iommu=", "gpu=", "hda-beside-nic")) and n != "bochs-display"]
ALL += ["nic=igb"]


class LaneMachine(unittest.TestCase):
    def test_the_swaps_and_no_lane_leaves_the_machine_as_it_was(self):
        plain = machine.devices("nat", "/s", None)
        self.assertIn("virtio-net-pci,netdev=net0", plain)
        self.assertIn("tpm-crb,tpmdev=tpm0", plain)
        swaps = lanes.plan(["nic=rtl8139", "tpm-tis", "vmware-svga"], "/d").swaps
        swapped = machine.devices("nat", "/s", None, swaps)
        self.assertEqual([a for a in swapped if a not in plain],
                         ["vmware-svga", "rtl8139,netdev=net0", "tpm-tis,tpmdev=tpm0"])
        self.assertEqual(len(swapped), len(plain))
        self.assertEqual(lanes.plan([], "/d"), lanes.Plan([], [], {}))

    def test_a_disk_is_made_once_and_then_kept(self):
        with tempfile.TemporaryDirectory() as d:
            p = lanes.plan(["sd"], os.path.join(d, "lanes"))
            lanes.make_images(p.images)
            (path, size), = p.images
            self.assertEqual(os.path.getsize(path), size)
            with open(path, "r+b") as f:
                f.write(b"kept")
            lanes.make_images(p.images)
            with open(path, "rb") as f:
                self.assertEqual(f.read(4), b"kept")

    def test_make_is_handed_the_same_lines(self):
        tool = os.path.join(os.path.dirname(HERE), "nonos-qemu-lanes")
        out = subprocess.run([sys.executable, tool, "args", "--dir", "/d", *ALL], check=True,
                             capture_output=True, text=True).stdout.split()
        self.assertEqual(out, lanes.plan(ALL, "/d").args)
        bad = subprocess.run([sys.executable, tool, "args", "--dir", "/d", "nope"], capture_output=True)
        self.assertNotEqual(bad.returncode, 0)
        # make cannot rewrite its machine, so it refuses an IOMMU lane rather than boot without one.
        mmu = subprocess.run([sys.executable, tool, "args", "--dir", "/d", "iommu=intel"],
                             capture_output=True, text=True)
        self.assertIn("nix run .#qemu -- --lane iommu=intel", mmu.stderr)


    def test_the_capture_is_the_guests_egress_and_its_ingress_apart(self):
        args = machine.capture("/t/a,b.pcap")
        self.assertEqual(args, ["-object", "filter-dump,id=egress0,netdev=net0,queue=rx,file=/t/a,,b.pcap",
                                "-object", "filter-dump,id=ingress0,netdev=net0,queue=tx,file=/t/a,,b-in.pcap"])
        self.assertEqual(machine.ingress_path("x/net"), "x/net-in.pcap")

if __name__ == "__main__":
    unittest.main()

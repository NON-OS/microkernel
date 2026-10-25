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

"""The IOMMU lanes' command lines; nothing here starts QEMU."""

import os
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import iommu, lanes, machine  # noqa: E402


def base(swaps=None):
    return ["qemu-system-x86_64", "-machine", "q35", *machine.disks(("/e", "/d.img"), None, False),
            *machine.devices("nat", "/sock", None, swaps), "-no-reboot"]


def values(cmd, flag):
    return [cmd[i + 1] for i, w in enumerate(cmd) if w == flag]


class Iommu(unittest.TestCase):
    def test_no_lane_leaves_the_machine_alone(self):
        self.assertEqual(iommu.apply(base(), None), base())

    def test_each_kind_sits_first_with_remapping_and_a_split_irqchip(self):
        for kind in iommu.UNITS:
            cmd = iommu.apply(base(), kind)
            self.assertEqual(values(cmd, "-machine"), ["q35,kernel-irqchip=split"])
            devices = values(cmd, "-device")
            self.assertTrue(devices[0].startswith(f"{kind}-iommu,"), devices[0])
            self.assertIn("intremap=on", devices[0])
            self.assertEqual(sum("-iommu" in d for d in devices), 1)

    def test_every_virtio_device_sends_dma_through_the_iommu(self):
        cmd = iommu.apply(base(), "intel")
        virtio = [d for d in values(cmd, "-device") if d.startswith("virtio-")]
        self.assertGreaterEqual(len(virtio), 4, "blk, net, rng and the GPU")
        for d in virtio:
            self.assertIn("iommu_platform=on", d)
            self.assertIn("disable-legacy=on", d)
        others = [d for d in values(cmd, "-device") if not d.startswith(("virtio-", "intel-"))]
        self.assertTrue(all("iommu_platform" not in d for d in others), others)

    def test_a_swapped_device_is_left_as_it_is(self):
        cmd = iommu.apply(base({"gpu": "vmware-svga", "nic": "e1000"}), "amd")
        devices = values(cmd, "-device")
        self.assertTrue({"vmware-svga", "e1000,netdev=net0"} <= set(devices), devices)

    def test_one_iommu_a_known_one(self):
        self.assertRaises(lanes.LaneError, lanes.plan, ["iommu=intel", "iommu=amd"], "/d")
        self.assertRaises(ValueError, iommu.apply, base(), "arm")
        self.assertEqual(lanes.plan(["iommu=amd"], "/d").swaps, {"iommu": "amd"})


if __name__ == "__main__":
    unittest.main()

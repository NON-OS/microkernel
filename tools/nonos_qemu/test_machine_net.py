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

"""--net off leaves the machine with no network device at all. Without
`-nic none` QEMU's q35 adds its default e1000e, and a boot meant to have no
network still spawned a NIC driver. Nothing here starts QEMU."""

import os
import sys
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.dirname(HERE))

from nonos_qemu import lanes, machine  # noqa: E402

NICS = ("virtio-net-pci", "e1000", "e1000e", "rtl8139", "pcnet", "vmxnet3", "tulip", "igb")


def nic_args(argv):
    return [a for a in argv if a.split(",")[0] in NICS or a.startswith(("user,id=net0", "netdev="))]


class NetOff(unittest.TestCase):
    def test_off_asks_for_no_nic_and_names_none(self):
        argv = machine.devices("off", None, None)
        self.assertEqual(nic_args(argv), [])
        self.assertNotIn("-netdev", argv)
        i = argv.index("-nic")
        self.assertEqual(argv[i + 1], "none", "q35's default NIC turned off")

    def test_off_with_a_nic_lane_still_has_none(self):
        argv = machine.devices("off", None, None, lanes.plan(["nic=e1000"], "/d").swaps)
        self.assertEqual(nic_args(argv), [])
        self.assertIn("none", argv)

    def test_make_off_names_none_too(self):
        mk = open(os.path.join(os.path.dirname(os.path.dirname(HERE)), "mk", "10-qemu.mk")).read()
        off = mk.split("ifeq ($(QEMU_NET_MODE),off)", 1)[1].split("else", 1)[0]
        self.assertIn("QEMU_NET := -nic none", off, "make's QEMU_NET_MODE=off")

    def test_nat_has_its_one_nic_and_no_none(self):
        argv = machine.devices("nat", None, None)
        self.assertIn("virtio-net-pci,netdev=net0", argv)
        self.assertNotIn("-nic", argv)


if __name__ == "__main__":
    unittest.main()

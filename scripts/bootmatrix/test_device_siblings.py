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
"""The HDA-beside-a-NIC device cell (device_table.py): the machine it asks
for, and that the refusal it must see is hda's, held by the e1000, so the
cell holds the network-wins ruling. No QEMU here. Run from the repository
root: python3 -m unittest discover -s scripts -p 'test_*.py'."""

import re
import unittest

from bootmatrix.device_table import DEVICE_CELLS, HDA_HELD, SIBLINGS
from bootmatrix.qemu import command
from bootmatrix.run import Paths

CELL = next(c for c in DEVICE_CELLS if c.name == "q35-iommu-hda-beside-e1000")


def argv():
    paths = Paths("qemu", {"eth": "esp"}, "code.fd", "vars.fd", "blk.img")
    return command(CELL, paths, "tcg", "s.log", "blk", "vars")


def function(model):
    """The PCI function the cell puts `model` at behind its root port."""
    line = next(a for a in SIBLINGS if a.startswith(f"-device {model},"))
    return re.search(r"addr=0\.(\d)", line).group(1)


class Siblings(unittest.TestCase):
    def test_both_functions_sit_behind_one_root_port_under_the_iommu(self):
        a = argv()
        first = a.index("intel-iommu,intremap=on,caching-mode=on")
        port = a.index("pcie-root-port,id=cellsib,chassis=1,slot=1")
        hda = a.index("intel-hda,bus=cellsib,addr=0.0,multifunction=on")
        nic = a.index("e1000,netdev=cellnet,bus=cellsib,addr=0.1")
        self.assertLess(first, port)
        self.assertLess(port, min(hda, nic))
        self.assertEqual(CELL.profile, "eth", "the image with both e1000 and hda")

    def test_the_refusal_is_hdas_and_the_e1000_holds_the_group(self):
        m = re.fullmatch(r"\[VT-D\] refused: 01:00\.(\d) reaches it through 01:00\.(\d) "
                         r"without ACS; driver\.(\w+) holds it", HDA_HELD)
        self.assertIsNotNone(m, HDA_HELD)
        member, hop, holder = m.groups()
        self.assertEqual(member, function("e1000"), "the held member is the NIC")
        self.assertEqual(hop, function("intel-hda"), "the claim refused is hda's")
        self.assertEqual(holder, "e1000_0")
        self.assertIn(HDA_HELD, CELL.expect)
        self.assertIn("[NET-CORE] bind: interface up on driver.e1000_0", CELL.expect)


if __name__ == "__main__":
    unittest.main()

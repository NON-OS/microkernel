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
"""The NIC device cells (device_table.py): each card on user networking, and
every cell under an IOMMU. No QEMU here. Run from the repository root:
python3 -m unittest discover -s scripts -p 'test_*.py'."""

import unittest

from bootmatrix import devices
from bootmatrix.device_table import DEVICE_CELLS, HELD_CELLS, nic
from bootmatrix.qemu import command
from bootmatrix.run import Paths


class NicCells(unittest.TestCase):
    def test_a_nic_cell_adds_its_card_on_user_networking(self):
        paths = Paths("qemu", {"eth": "esp"}, "code.fd", "vars.fd", "blk.img")
        argv = command(nic("rtl8139"), paths, "tcg", "s.log", "blk", "vars")
        self.assertIn("rtl8139,netdev=cellnet", argv)
        self.assertIn("user,id=cellnet", argv)
        self.assertEqual([c.name for c in DEVICE_CELLS if "-nic-" in c.name],
                         ["q35-iommu-nic-e1000", "q35-iommu-nic-rtl8139"])

    def test_the_igb_cell_fails_on_any_uncovered_claim(self):
        igb = next(c for c in HELD_CELLS if c.name == "q35-iommu-nic-igb")
        self.assertNotIn(igb, DEVICE_CELLS, "held until igb has trust files")
        served = "\n".join(igb.expect) + "\n"
        self.assertEqual(devices.verdict(igb, served), [])
        bad = devices.verdict(igb, served + "[VT-D] refused: no remapping unit confines it\n")
        self.assertEqual(bad, ["device cell: a line 'no remapping unit confines it'"])
        self.assertEqual(nic("e1000").forbid, ())

    def test_every_cell_boots_under_an_iommu(self):
        # Fail closed (confine/decide.rs): with no remapping unit every DMA
        # claim is refused, so a cell without one could never pass.
        for cell in DEVICE_CELLS:
            self.assertEqual(cell.iommu, "intel-iommu", cell.name)


if __name__ == "__main__":
    unittest.main()

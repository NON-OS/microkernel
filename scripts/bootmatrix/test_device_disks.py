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
"""The NVMe and HDA device cells (device_table.py), and the scratch disk the
runner makes for a cell that names one. No QEMU here. Run from the
repository root: python3 -m unittest discover -s scripts -p 'test_*.py'."""

import tempfile
import unittest
from pathlib import Path

from bootmatrix import devices
from bootmatrix.cell import Cell
from bootmatrix.device_table import DEVICE_CELLS
from bootmatrix.qemu import command
from bootmatrix.run import Paths

CELLS = {c.name: c for c in DEVICE_CELLS}


class DiskCells(unittest.TestCase):
    def test_the_scratch_disk_reaches_qemu_and_is_sized(self):
        cell = Cell("t", "up", "q35", 1, devices=("-drive file={disk},if=none,id=d",), expect=("x",))
        with tempfile.TemporaryDirectory() as tmp:
            disks = devices.scratch_disks(cell, Path(tmp) / "t-1")
            self.assertEqual(disks[0].stat().st_size, devices.DISK_BYTES)
            paths = Paths("qemu", {"up": "esp"}, "code.fd", "vars.fd", "blk.img")
            argv = command(cell, paths, "tcg", "s.log", "blk", "vars", disks)
            self.assertIn(f"file={disks[0]},if=none,id=d", argv)
            self.assertNotIn("{disk}", " ".join(argv))

    def test_a_cell_with_no_disk_gets_none(self):
        with tempfile.TemporaryDirectory() as tmp:
            self.assertEqual(devices.scratch_disks(CELLS["q35-iommu-hda"], Path(tmp) / "h"), [])

    def test_nvme_must_name_the_scratch_disks_size_and_serve_it(self):
        cell = CELLS["q35-iommu-nvme"]
        self.assertIn(f"nvme: nsid 1: {devices.DISK_BYTES // 512} LBAs of 512 bytes", cell.expect)
        self.assertIn(", block I/O up", cell.expect)
        self.assertTrue(any("{disk}" in a for a in cell.devices))

    def test_no_cell_waits_on_a_line_an_amnesic_boot_never_writes(self):
        # Every image boots on the loader's copy of the store: no disk is asked.
        for cell in DEVICE_CELLS:
            self.assertFalse(any(e.startswith("[BLOCK] asked") for e in cell.expect), cell.name)
        self.assertNotIn("q35-iommu-ahci", CELLS, "AHCI waits for hw/debug-grants")
        self.assertEqual(CELLS["q35-iommu-hda"].expect, ("[HDA] ready: ", "[AUDIO] output driver.hda0"))


if __name__ == "__main__":
    unittest.main()

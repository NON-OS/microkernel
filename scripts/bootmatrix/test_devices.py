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
"""The device cells' verdict; nothing here starts QEMU. Run with
python3 -m unittest discover -s scripts -p 'test_*.py'."""

import unittest

from bootmatrix import devices
from bootmatrix.device_table import DEVICE_CELLS, nic
from bootmatrix.cell import Cell


def served(cell):
    """A log in which the cell's driver wrote every line it must."""
    return "\n".join(("Handoff OK",) + cell.expect) + "\n"


class DeviceCells(unittest.TestCase):
    def test_every_cell_names_the_lines_it_must_see(self):
        self.assertTrue(DEVICE_CELLS)
        for cell in DEVICE_CELLS:
            self.assertTrue(cell.expect, cell.name)
            # A device of its own, or one the machine has built in (q35's AHCI).
            self.assertTrue(not cell.devices or any("-device" in a for a in cell.devices), cell.name)

    def test_a_log_with_every_line_passes(self):
        for cell in DEVICE_CELLS:
            self.assertEqual(devices.verdict(cell, served(cell)), [], cell.name)

    def test_each_line_dropped_alone_fails_naming_it(self):
        # The mutant: one marker removed from a log that held them all.
        for cell in DEVICE_CELLS:
            for line in cell.expect:
                bad = devices.verdict(cell, served(cell).replace(line, ""))
                self.assertEqual(bad, [f"device cell: no line {line!r}"], cell.name)

    def test_the_virtio_nic_bound_instead_fails(self):
        cell = nic("e1000")
        log = served(cell) + "[NET-CORE] bind: interface up on driver.virtio_net0\n"
        self.assertIn("device cell: net.core bound the virtio NIC, not the card under test",
                      devices.verdict(cell, log))

    def test_a_plain_cell_asks_nothing(self):
        self.assertEqual(devices.verdict(Cell("q35-up", "up", "q35", 1), ""), [])

if __name__ == "__main__":
    unittest.main()

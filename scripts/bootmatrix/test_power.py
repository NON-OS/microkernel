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
"""The power button verdict; nothing here starts QEMU. Run with
python3 -m unittest discover -s scripts -p 'test_power.py'."""

import unittest

from bootmatrix import power
from bootmatrix.cell import Cell

Q35 = Cell("q35-iommu-up", "up", "q35", 1, iommu="intel-iommu")
PC = Cell("i440fx-up", "up", "pc", 1)
ARMED = "[ACPI] sci irq=9 device=0x000000000000000D\n[ACPI] power button: SCI line masked, PM1_EN=0x0000000000000100\n"
# The line 551520c30 printed on standard-dev: the arm call ran at the broker's
# seeding, before routing (privacy/p05-evidence 23aa9e816, serial line 101).
EARLY = ("[ACPI] sci irq=9 device=0x000000000000000D\n"
         "[ACPI] power button: the SCI line could not be masked, so PWRBTN_EN stays clear\n")


class PowerButton(unittest.TestCase):
    def test_an_armed_button_passes(self):
        self.assertEqual(power.verdict(Q35, ARMED), [])
        self.assertEqual(power.verdict(Q35, ARMED.replace("0x0000000000000100", "0x0121")), [])

    def test_mutant_armed_before_routing_fails_the_cell(self):
        self.assertEqual(len(power.verdict(Q35, EARLY)), 1)

    def test_no_line_at_all_fails(self):
        self.assertEqual(len(power.verdict(Q35, "[ACPI] sci irq=9 device=0xD\n")), 1)

    def test_an_enable_that_did_not_stick_fails(self):
        self.assertEqual(len(power.verdict(Q35, ARMED.replace("0100", "0000"))), 1)

    def test_only_q35_cells_are_held_to_it(self):
        self.assertEqual(power.verdict(PC, ""), [])


if __name__ == "__main__":
    unittest.main()

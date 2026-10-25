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
"""The power button as a q35 cell must arm it (P-05).

QEMU's PM1 latches a power button press only while PWRBTN_EN is set
(hw/acpi/core.c, acpi_pm1_evt_power_down), so a q35 boot that does not arm
the poll never sees the button. The kernel arms it once device interrupt
routing is up and says so with the enable register's read-back; the line
must be there with bit 8 set. Armed any earlier, the SCI's IO-APIC line is
not yet known, the mask is refused, and the enable stays clear: that boot
printed a refusal in place of this line, and fails here.
"""

import re

ARMED = re.compile(r"\[ACPI\] power button: SCI line masked, PM1_EN=(?:0x)?([0-9A-Fa-f]+)")
PWRBTN_EN = 1 << 8


def verdict(cell, text):
    """Why the cell's log does not show the button armed, if it does not."""
    if cell.machine != "q35":
        return []
    armed = ARMED.search(text)
    if not armed:
        return ["the power button was never armed (no 'SCI line masked, PM1_EN=' line)"]
    if int(armed.group(1), 16) & PWRBTN_EN == 0:
        return [f"PM1_EN read back {armed.group(1)} after arming, PWRBTN_EN clear"]
    return []

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
"""The device cells: one QEMU model of a part real machines carry, attached
to a boot that must then show the driver serving it, not merely spawned.

Each cell names the lines that prove it, and the boot fails when any is
missing (HANDOFF/hardware/README.md: "booted" means this, no less). A cell with
a NIC boots the ethernet image (`eth`, nonos-mk-ethernet-prod) with no virtio
NIC beside its card, so net.core has only the card under test to bind and
its lease comes through that card's receive path.
"""


def arguments(cell):
    """The QEMU arguments for the cell's devices."""
    return [word for group in cell.devices for word in group.split()]


def verdict(cell, text):
    """Every line the cell's driver had to write and did not, and a virtio NIC
    bound where the card under test should have been."""
    bad = [f"device cell: no line {line!r}" for line in cell.expect if line not in text]
    if cell.expect and "bind: interface up on driver.virtio_net0" in text:
        bad.append("device cell: net.core bound the virtio NIC, not the card under test")
    return bad

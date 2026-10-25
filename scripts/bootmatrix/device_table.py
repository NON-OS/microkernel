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
"""The device cells (devices.py says what one is). Each attaches a QEMU
model of a part real machines carry, or names one the machine has built in,
and the lines its driver and the stack must write once they serve it."""

from .cell import Cell

USER_NET = "-netdev user,id=cellnet"

# HDA and the NIC as functions 0 and 1 of one device behind a root port, no
# ACS between them, as an Intel PCH puts HDA (1f.3) beside its I219 (1f.6): one
# isolation group, which the broker gives to one capsule. The NICs claim
# before hda, so the network wins and hda is refused naming its holder (the
# Lead's ruling, HANDOFF/hardware/platform.md). The port's bus is 01.
SIBLINGS = ("-device pcie-root-port,id=cellsib,chassis=1,slot=1",
            "-audiodev none,id=cellsnd",
            "-device intel-hda,bus=cellsib,addr=0.0,multifunction=on",
            "-device hda-duplex,audiodev=cellsnd",
            "-device e1000,netdev=cellnet,bus=cellsib,addr=0.1")
HDA_HELD = "[VT-D] refused: 01:00.1 reaches it through 01:00.0 without ACS; driver.e1000_0 holds it"

DEVICE_CELLS = [
    Cell("q35-iommu-hda-beside-e1000", "eth", "q35", 1, iommu="intel-iommu",
         devices=(USER_NET, *SIBLINGS),
         expect=("[NET-CORE] bind: interface up on driver.e1000_0", "[NET-CORE] lease ", HDA_HELD)),
]

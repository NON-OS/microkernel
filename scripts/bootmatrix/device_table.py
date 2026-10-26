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
from .devices import DISK_BYTES

USER_NET = "-netdev user,id=cellnet"


def nic(model, forbid=()):
    """A NIC cell: the card bound, and a DHCP lease taken through it. Under
    VT-d, since a DMA claim no remapping unit confines is refused."""
    return Cell(f"q35-iommu-nic-{model}", "eth", "q35", 1, iommu="intel-iommu", forbid=forbid,
                devices=(USER_NET, f"-device {model},netdev=cellnet"),
                expect=(f"[NET-CORE] bind: interface up on driver.{model}_0", "[NET-CORE] lease "))


# HDA and the NIC as functions 0 and 1 of one device behind a root port, no
# ACS between them, as an Intel PCH puts HDA (1f.3) beside its I219 (1f.6): one
# isolation group, which the broker gives to one capsule. The NICs claim
# before hda, so the network wins and hda is refused naming its holder (the
# the ruling, the design notes). The port's bus is 01.
SIBLINGS = ("-device pcie-root-port,id=cellsib,chassis=1,slot=1",
            "-audiodev none,id=cellsnd",
            "-device intel-hda,bus=cellsib,addr=0.0,multifunction=on",
            "-device hda-duplex,audiodev=cellsnd",
            "-device e1000,netdev=cellnet,bus=cellsib,addr=0.1")
HDA_HELD = "[VT-D] refused: 01:00.1 reaches it through 01:00.0 without ACS; driver.e1000_0 holds it"

# The scratch disk is 64 MiB: 131072 sectors, which the driver names once it
# read the namespace's size. Every image boots on the loader's copy of the
# store, so the block layer asks no disk, and each cell is held to its
# driver's own lines (Debug, optional in their manifests).
SECTORS = DISK_BYTES // 512

# Held until the release seal: q35-iommu-ahci (hw/debug-grants: no line yet);
# q35-smbus-novtd (no IOMMU, H-02b; smbus in an image, item 52), to see "port-only
# with no remapping unit over it: Bus Master off, no DMA grant", "driver.smbus:
# i801 0x8086:0x2930 up, spd answered", and "no remapping unit confines it".
# igb's VFs make QEMU's DMAR name every endpoint, past 16: held for its keys.
UNCOVERED = "no remapping unit confines it"
HELD_CELLS = [nic("igb", forbid=(UNCOVERED,))]

DEVICE_CELLS = [
    Cell("q35-iommu-nvme", "up", "q35", 1, iommu="intel-iommu",
         devices=("-drive file={disk},if=none,id=celldisk,format=raw",
                  "-device nvme,serial=nonos-cell,drive=celldisk"),
         expect=(f"nvme: nsid 1: {SECTORS} LBAs of 512 bytes",
                 "nvme: serving controller", ", block I/O up")),
    Cell("q35-iommu-hda", "up", "q35", 1, iommu="intel-iommu",
         devices=("-audiodev none,id=cellsnd", "-device intel-hda",
                  "-device hda-duplex,audiodev=cellsnd"),
         expect=("[HDA] ready: ", "[AUDIO] output driver.hda0")),
    nic("e1000"),
    nic("rtl8139"),
    Cell("q35-iommu-hda-beside-e1000", "eth", "q35", 1, iommu="intel-iommu",
         devices=(USER_NET, *SIBLINGS),
         expect=("[NET-CORE] bind: interface up on driver.e1000_0", "[NET-CORE] lease ", HDA_HELD)),
]

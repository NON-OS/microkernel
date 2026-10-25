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
"""One cell: a QEMU configuration and what its boot must show."""

from dataclasses import dataclass


@dataclass(frozen=True)
class Cell:
    name: str
    profile: str
    machine: str
    cpus: int
    # The IOMMU QEMU presents: "" for none, "intel-iommu" or "amd-iommu".
    iommu: str = ""
    # Kill QEMU once the store is serving, then boot the same disk again and
    # require it to come back clean. Crash consistency of the block store.
    kill: bool = False
    # The store disk on a USB mass-storage stick on the xHCI controller, in
    # place of virtio-blk, with logical blocks of this many bytes; 0 keeps
    # virtio-blk. The stick is served by driver.xhci0 and driver.usb_msc0,
    # the path a live boot from a stick takes on hardware.
    usb_block: int = 0
    # Add a transitional virtio-rng the kernel must refuse (K5, virtio.py).
    legacy_virtio: bool = False

    # A device cell: the QEMU arguments that attach its device, and the lines
    # its driver and the stack must write once they serve it (devices.py).
    devices: tuple = ()
    expect: tuple = ()
    # Lines that fail the cell wherever they appear (devices.py).
    forbid: tuple = ()

    def wants_smp_proof(self):
        return self.cpus > 1

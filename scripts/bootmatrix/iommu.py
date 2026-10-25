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

"""What a serial log has to say about the IOMMU for its cell.

Every cell carries the [IOMMU] posture line its QEMU device calls for. A cell
with an intel-iommu must not hear that DMA is unrestricted, must hear its
devices blocked until claimed, and must never take the no-IOMMU posture. A
cell with no IOMMU, or an amd-iommu the kernel does not put in service, must
take that posture by its own line and print it in the posture line's tail,
so a machine running with its devices unconfined always says so.
"""

import re

UNRESTRICTED = "DMA is unrestricted"
REMAPPED = "[VT-D] enumerated devices blocked until claimed"
AMD_VI_NAMED = "[AMD-VI] IVRS present"
# Every posture line carries the unconfined count beside enforcing=. The line
# is printed again each time the count changes, so the last one is current.
POSTURE = re.compile(r"\[IOMMU\] (\S+) present, enforcing=(\d), unconfined grants=(\d+)([^\n]*)")
# The posture each IOMMU QEMU can present must produce: (vendor, enforcing).
EXPECTED_POSTURE = {"": ("none", "0"), "intel-iommu": ("intel-vt-d", "1"), "amd-iommu": ("amd-vi", "0")}
# The no-IOMMU posture's own line, said once when the boot takes it.
TAKEN = re.compile(r"\[IOMMU\] posture=dma-unconfined reason=(\S+) profile=")
TAIL = ", posture=dma-unconfined"


def judge(cell, text, reached):
    bad = []
    postures = POSTURE.findall(text)
    if reached and not postures:
        bad.append("no [IOMMU] posture line")
    vendor, enforcing = EXPECTED_POSTURE[cell.iommu]
    for seen_vendor, seen_enforcing, _, _ in postures:
        if (seen_vendor, seen_enforcing) != (vendor, enforcing):
            bad.append(f"posture {seen_vendor} enforcing={seen_enforcing}, "
                       f"expected {vendor} enforcing={enforcing}")
            break
    taken = TAKEN.search(text)
    if enforcing == "1":
        if taken or any(TAIL in p[3] for p in postures):
            bad.append("the no-IOMMU posture was taken with a unit in service")
        if UNRESTRICTED in text:
            bad.append("kernel reports DMA is unrestricted with an IOMMU present")
        if REMAPPED not in text:
            bad.append("VT-d never reported devices remapped")
    elif reached:
        if not taken:
            bad.append("no IOMMU in service and no posture=dma-unconfined line")
        if postures and TAIL not in postures[-1][3]:
            bad.append("the last posture line does not name posture=dma-unconfined")
    if cell.iommu == "amd-iommu" and AMD_VI_NAMED not in text:
        bad.append("kernel never named the AMD-Vi hardware it does not drive")
    return bad

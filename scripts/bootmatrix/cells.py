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
"""The machines the kernel has to boot on.

One cell is one QEMU configuration. The table is the claim: every shipping
image boots on each of these, every time, and the runner's job is to find the
one that does not. `profile` names which built ESP the cell boots, since the
multiprocessor image is a separate build of the same tree.
"""

from .cell import Cell
from .device_table import DEVICE_CELLS


CELLS = [
    Cell("q35-up", "up", "q35", 1),
    Cell("q35-smp2", "smp", "q35", 2),
    Cell("q35-smp4", "smp", "q35", 4),
    Cell("q35-smp8", "smp", "q35", 8),
    Cell("i440fx-up", "up", "pc", 1),
    Cell("i440fx-smp4", "smp", "pc", 4),
    Cell("q35-iommu-up", "up", "q35", 1, iommu="intel-iommu"),
    Cell("q35-iommu-smp4", "smp", "q35", 4, iommu="intel-iommu"),
    Cell("q35-amdvi-up", "up", "q35", 1, iommu="amd-iommu"),
    Cell("q35-iommu-legacy-virtio", "up", "q35", 1, "intel-iommu", usb_block=512, legacy_virtio=True),
    Cell("q35-kill-reboot", "up", "q35", 1, kill=True),
    Cell("q35-usb-stick", "smp", "q35", 4, usb_block=512),
    Cell("q35-usb-stick-4k", "up", "q35", 1, usb_block=4096),
    *DEVICE_CELLS,
]


def select(names):
    """The cells named, in table order, or all of them when nothing is named."""
    if not names:
        return list(CELLS)
    known = {c.name: c for c in CELLS}
    missing = [n for n in names if n not in known]
    if missing:
        raise SystemExit(f"boot-matrix: no such cell {', '.join(missing)}")
    return [c for c in CELLS if c.name in names]

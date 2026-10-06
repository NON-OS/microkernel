// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Whether a device record is an NVMe controller with a usable register BAR.

use nonos_libc::{DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::{CLASS_BLOCK, NVME_BAR_INDEX, NVME_BAR_MIN_SIZE};

const PCI_CLASS_STORAGE: u8 = 0x01;
const PCI_SUBCLASS_NVM: u8 = 0x08;
const PCI_PROGIF_NVME: u8 = 0x02;

pub(super) fn is_nvme(r: &DeviceRecord) -> bool {
    r.bus_kind == BUS_KIND_PCI
        && r.class == CLASS_BLOCK
        && r.pci_class == PCI_CLASS_STORAGE
        && r.pci_subclass == PCI_SUBCLASS_NVM
        && r.pci_progif == PCI_PROGIF_NVME
}

pub(super) fn has_register_bar(r: &DeviceRecord) -> bool {
    let bar = r.bars[NVME_BAR_INDEX as usize];
    r.bar_count > NVME_BAR_INDEX && bar.kind == BAR_KIND_MMIO && bar.size >= NVME_BAR_MIN_SIZE
}

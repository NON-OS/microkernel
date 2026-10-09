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
//! Which PCI functions are HD Audio controllers this driver can run.

use nonos_libc::{DeviceRecord, BAR_KIND_MMIO, BUS_KIND_PCI};

use crate::constants::{CLASS_AUDIO, HDA_BAR_INDEX, HDA_BAR_MIN_SIZE, PCI_CLASS_MULTIMEDIA};
use crate::controller::intel::hda_controller;
use crate::controller::sst::intel_sst;

/// An HD Audio controller with its register BAR: class 0x0403 from any
/// vendor, or 0x0401 from Intel unless it is an SST engine, whose BAR0 is
/// no HD Audio register set (`controller::sst`). The interrupt routing is not asked here:
/// `setup::irq` picks INTx, MSI-X or polling for it.
pub(super) fn is_candidate(r: &DeviceRecord) -> bool {
    let bar = r.bars[HDA_BAR_INDEX as usize];
    r.bus_kind == BUS_KIND_PCI
        && r.class == CLASS_AUDIO
        && r.pci_class == PCI_CLASS_MULTIMEDIA
        && hda_controller(r.vendor, r.pci_subclass)
        && !intel_sst(r.vendor, r.device)
        && r.bar_count > HDA_BAR_INDEX
        && bar.kind == BAR_KIND_MMIO
        && bar.size >= HDA_BAR_MIN_SIZE
}

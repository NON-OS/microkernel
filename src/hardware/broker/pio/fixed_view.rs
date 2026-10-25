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

//! The interrupt a bind is validated against. A channel in compatibility
//! mode interrupts on its fixed ISA line whatever the function's pin and
//! line registers say (PIIX reports no pin at all), so a bind that names
//! that line from the function's claimant is checked as if the function
//! were wired to it. Every other bind sees the record unchanged.

use super::fixed_line::owns_line;
use crate::hardware::broker::legacy::compat::ide;
use crate::hardware::broker::{BusKind, DeviceRecord};

pub(in crate::hardware::broker) fn intx_view(dev: &DeviceRecord, irq_source: u32) -> (u8, u8) {
    let f = ide(dev.pci_class, dev.pci_subclass, dev.pci_progif);
    let (primary, secondary) = (f.bars[0].is_some(), f.bars[2].is_some());
    if dev.bus_kind == BusKind::Pci as u8 && owns_line(primary, secondary, irq_source) {
        (1, irq_source as u8)
    } else {
        (dev.irq_pin, dev.irq_line)
    }
}

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

//! Giving a device's MSI or MSI-X vector a remapped entry, and taking it
//! back. This is what an interrupt driver calls instead of composing a
//! compatibility format message: program the returned address and data into
//! the capability or the MSI-X table entry, and keep the index to release.

use super::irte::{encode, Route};
use super::msi::{remapped, MsiMessage};
use super::slots::{give, take};
use super::table::{is_remapping, TABLE};
use super::write::write;
use crate::arch::x86_64::iommu::device::bdf_to_source_id;
use crate::arch::x86_64::iommu::types::VtdError;

/// An edge-triggered `vector` on the CPU with physical APIC id `destination`,
/// for interrupts from `bus:device.function` only. Returns the entry index
/// and the message to program.
pub fn route_msi(
    (bus, device, function): (u8, u8, u8),
    vector: u8,
    destination: u32,
) -> Result<(u16, MsiMessage), VtdError> {
    if !is_remapping() {
        return Err(VtdError::InterruptRemappingOff);
    }
    // Its messages may carry a bridge's id, which an entry checked against
    // its own would block: it gets no entry (L54).
    if !super::super::own_id_read::own_id(bus, device, function) {
        return Err(VtdError::SharedRequesterId);
    }
    let source = bdf_to_source_id(bus, device, function).as_u16();
    let index = route_entry(Route { vector, destination, source, level: false })?;
    Ok((index, remapped(index)))
}

/// An entry for `route`, taken only by interrupts from its source; its index
/// is the handle an MSI message or an IO-APIC entry names.
pub fn route_entry(route: Route) -> Result<u16, VtdError> {
    if !is_remapping() {
        return Err(VtdError::InterruptRemappingOff);
    }
    let entry = encode(route, false).ok_or(VtdError::DestinationTooWide)?;
    let mut table = TABLE.lock();
    let index = take(&mut table.slots).ok_or(VtdError::RemapTableFull)?;
    if let Err(e) = write(table.phys, index, entry) {
        give(&mut table.slots, index);
        return Err(e);
    }
    Ok(index)
}

/// Gives back any entry `route_entry` or `route_msi` took.
pub fn release_msi(index: u16) -> Result<(), VtdError> {
    let mut table = TABLE.lock();
    if !give(&mut table.slots, index) {
        return Err(VtdError::DeviceNotAttached);
    }
    write(table.phys, index, [0, 0])
}

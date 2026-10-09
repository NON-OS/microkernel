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
    let source = bdf_to_source_id(bus, device, function).as_u16();
    let route = Route { vector, destination, source, level: false };
    let entry = encode(route, false).ok_or(VtdError::DestinationTooWide)?;
    let mut table = TABLE.lock();
    let index = take(&mut table.slots).ok_or(VtdError::RemapTableFull)?;
    if let Err(e) = write(table.phys, index, entry) {
        give(&mut table.slots, index);
        return Err(e);
    }
    Ok((index, remapped(index)))
}

/// Withdraw an entry. The device must already be stopped from raising it;
/// an interrupt arriving after this faults with reason 0x22 and is dropped.
pub fn release_msi(index: u16) -> Result<(), VtdError> {
    let mut table = TABLE.lock();
    if !give(&mut table.slots, index) {
        return Err(VtdError::DeviceNotAttached);
    }
    write(table.phys, index, [0, 0])
}

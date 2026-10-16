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

//! A remapped entry for a claimed device's MSI or MSI-X vector, and taking
//! it back. The broker programs the returned address and data into the
//! capability or the table entry, and keeps the handle to release.

use super::super::coverage::unit_covers;
use super::super::domain::device_id;
use super::super::error::AmdViError;
use super::super::flush::flush_interrupts;
use super::super::intr_format::{irte, msi_data, MSI_ADDRESS};
use super::handles::{issue, take};
use super::irte_write::write_irte;
use super::slots;
use super::table::{table_of, TABLES};
use crate::arch::x86_64::acpi::parser::other::amd_iommu_aliased;

/// A fixed, edge `vector` on the CPU with APIC id `dest`, raised by
/// `bus:device.function` alone: (handle, MSI address, MSI data).
pub fn route_msi(
    (bus, device, function): (u8, u8, u8),
    vector: u8,
    dest: u8,
) -> Result<(u16, u64, u32), AmdViError> {
    // A device no enforcing unit covers would send the index as a raw
    // vector: 17 is #AC, 32 and up are the kernel's own (L46).
    if !unit_covers(bus, device, function) {
        return Err(AmdViError::NotEnforcing);
    }
    let id = device_id(bus, device, function);
    // Its interrupts would share a table with another id's (L47, L49).
    if amd_iommu_aliased(id) {
        return Err(AmdViError::Aliased);
    }
    let mut tables = TABLES.lock();
    let table = table_of(&mut tables, id)?;
    let index = slots::take(&mut table.slots).ok_or(AmdViError::InterruptTableFull)?;
    let written =
        write_irte(table.phys, index, irte(vector, dest)).and_then(|_| flush_interrupts(id));
    let handle = written.and_then(|_| issue(id, index).ok_or(AmdViError::InterruptTableFull));
    if handle.is_err() {
        let _ = write_irte(table.phys, index, 0);
        slots::give(&mut table.slots, index);
    }
    Ok((handle?, MSI_ADDRESS, msi_data(index)))
}

/// Withdraw the entry behind `handle`. The device must already be stopped
/// from raising it; one arriving after finds the entry clear and is blocked.
pub fn release_msi(handle: u16) -> Result<(), AmdViError> {
    let (id, index) = take(handle).ok_or(AmdViError::NotPresent)?;
    let mut tables = TABLES.lock();
    let table = tables.get_mut(&id).ok_or(AmdViError::NotPresent)?;
    write_irte(table.phys, index, 0)?;
    slots::give(&mut table.slots, index);
    flush_interrupts(id)
}

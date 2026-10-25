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

//! An entry at index `vector` in a requester id's own table, for a source
//! whose messages carry the vector itself: an I/O APIC's lines, whose
//! redirection entries stay in compatibility format so a level line's EOI
//! still matches, or a broker vector sent as the plain message. Such a table
//! is never also handed out by handle (table.rs).

use super::super::error::AmdViError;
use super::super::flush::flush_interrupts;
use super::super::intr_format::irte;
use super::irte_write::write_irte;
use super::table::{table_of, TABLES};

/// Every unit in service, over a device table in which every entry refuses
/// interrupts until it is given a table of its own.
pub fn is_remapping() -> bool {
    super::super::units::is_enforcing()
}

/// `vector` on the CPU with APIC id `dest`, for interrupts from `id` only.
/// The entry is the vector's own: the next route of that vector rewrites it.
pub fn route(id: u16, vector: u8, dest: u8) -> Result<(), AmdViError> {
    if !is_remapping() {
        return Err(AmdViError::NotEnforcing);
    }
    // Entry 0 is what a device whose message was never written names.
    if vector == 0 {
        return Err(AmdViError::OutOfRange);
    }
    let mut tables = TABLES.lock();
    let table = table_of(&mut tables, id, true)?;
    write_irte(table.phys, vector as u16, irte(vector, dest))?;
    table.slots[vector as usize / 64] |= 1 << (vector % 64);
    flush_interrupts(id)
}

/// Clear `vector`'s entry; one raised after finds it clear and is blocked.
pub fn release(id: u16, vector: u8) -> Result<(), AmdViError> {
    let mut tables = TABLES.lock();
    let table = tables.get_mut(&id).filter(|t| t.by_vector);
    let table = table.ok_or(AmdViError::DeviceNotAttached)?;
    write_irte(table.phys, vector as u16, 0)?;
    super::slots::give(&mut table.slots, vector as u16);
    flush_interrupts(id)
}

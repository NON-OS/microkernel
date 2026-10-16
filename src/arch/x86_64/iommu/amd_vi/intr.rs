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

//! Each device's interrupt remapping table: made at its first bind,
//! never freed, an entry per vector, each written (or cleared at release)
//! and the units' cached copies dropped before the caller goes on.

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{fence, Ordering};
use spin::Mutex;

use super::command::invalidate_interrupt_table;
use super::devtab::{read_dte, write_dte};
use super::error::AmdViError;
use super::flush::{flush_device, submit_all};
use super::intr_rule::{irte, remapped};
use crate::arch::x86_64::iommu::tables::frame::{allocate_table, entries_mut};

static TABLES: Mutex<Vec<(u16, u64)>> = Mutex::new(Vec::new());

/// Every unit in service, over a device table whose every entry has IV.
pub fn is_remapping() -> bool {
    super::units::is_enforcing()
}

pub fn route(id: u16, vector: u8, dest: u8) -> Result<(), AmdViError> {
    if !is_remapping() {
        return Err(AmdViError::NotEnforcing);
    }
    let table = table_of(id)?;
    set(id, table, vector, irte(vector, dest))
}

pub fn release(id: u16, vector: u8) -> Result<(), AmdViError> {
    let table = TABLES.lock().iter().find(|t| t.0 == id).map(|t| t.1);
    match table {
        Some(table) => set(id, table, vector, 0),
        None => Err(AmdViError::DeviceNotAttached),
    }
}

fn table_of(id: u16) -> Result<u64, AmdViError> {
    let mut tables = TABLES.lock();
    if let Some(&(_, t)) = tables.iter().find(|t| t.0 == id) {
        return Ok(t);
    }
    let table = allocate_table().map_err(|_| AmdViError::NoFrames)?;
    let entry = read_dte(id)?;
    write_dte(id, [entry[0], entry[1], remapped(table), entry[3]])?;
    flush_device(id)?;
    tables.push((id, table));
    Ok(table)
}

fn set(id: u16, table: u64, vector: u8, value: u32) -> Result<(), AmdViError> {
    let slots = entries_mut(table).map_err(|_| AmdViError::TableUnreachable)?;
    let (slot, shift) = (vector as usize / 2, (vector as usize % 2) * 32);
    slots[slot] = (slots[slot] & !(0xFFFF_FFFF << shift)) | ((value as u64) << shift);
    fence(Ordering::SeqCst);
    submit_all(&[invalidate_interrupt_table(id)])
}

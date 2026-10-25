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

//! Each remapped device's own table, made with its first remapped vector.
//! Kept once made: turning remapping off again would let the device send
//! any interrupt, so a released device keeps an empty table and raises
//! nothing until its next claim writes an entry.

use alloc::collections::BTreeMap;
use spin::Mutex;

use super::super::devtab::update_dte;
use super::super::error::AmdViError;
use super::super::flush::flush_interrupts;
use super::super::intr_format::remapped_quadword;
use super::slots::{Slots, FRESH};
use crate::memory::phys::{alloc_contiguous, AllocFlags};

pub(super) struct Table {
    pub(super) phys: u64,
    pub(super) slots: Slots,
    /// Indexed by vector (by_vector.rs), never by handle (route.rs).
    pub(super) by_vector: bool,
}

pub(super) static TABLES: Mutex<BTreeMap<u16, Table>> = Mutex::new(BTreeMap::new());

/// `device`'s table, made, named in its entry and flushed on first use;
/// refused when it is indexed the other way.
pub(super) fn table_of(
    tables: &mut BTreeMap<u16, Table>,
    device: u16,
    by_vector: bool,
) -> Result<&mut Table, AmdViError> {
    if !tables.contains_key(&device) {
        let phys = alloc_contiguous(1, AllocFlags::ZERO).ok_or(AmdViError::NoFrames)?;
        let quadword = remapped_quadword(phys);
        update_dte(device, |mut entry| {
            entry[2] = quadword;
            entry
        })?;
        tables.insert(device, Table { phys, slots: FRESH, by_vector });
        flush_interrupts(device)?;
    }
    tables.get_mut(&device).filter(|t| t.by_vector == by_vector).ok_or(AmdViError::Aliased)
}

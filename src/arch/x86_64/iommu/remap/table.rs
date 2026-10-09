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

//! The one interrupt remapping table every unit is pointed at, as one root
//! table serves every unit's DMA: a handle names the same entry whichever
//! unit the device sits behind, and the entry's source id check keeps other
//! devices out of it.

use core::sync::atomic::{AtomicBool, Ordering};

use spin::Mutex;

use super::slots::{Slots, SLOT_WORDS};

pub(super) struct Table {
    /// The entry page, or zero before interrupt remapping is set up.
    pub phys: u64,
    pub slots: Slots,
}

pub(super) static TABLE: Mutex<Table> = Mutex::new(Table { phys: 0, slots: [0; SLOT_WORDS] });

/// Set once every unit acknowledged IRE with this table installed.
pub(super) static ACTIVE: AtomicBool = AtomicBool::new(false);

/// Whether interrupts can be routed through remapped entries. Until this
/// holds a driver programs compatibility format MSI as before, which the
/// units pass through while CFI is set.
pub fn is_remapping() -> bool {
    ACTIVE.load(Ordering::Acquire)
}

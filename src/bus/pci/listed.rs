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

//! How far the PCI table is whole (L68). It is filled in ascending bus order
//! and stops at its cap, so every function on a bus below the first one left
//! out is listed. A bridge sits on a bus below its secondary bus, so for any
//! bus up to that one, every bridge that could hold it is listed.

use core::sync::atomic::{AtomicU16, Ordering};

/// The bus of the first function left out, past every bus when none was.
static FIRST_LEFT_OUT: AtomicU16 = AtomicU16::new(u16::MAX);

pub(super) fn left_out_on(bus: u8) {
    FIRST_LEFT_OUT.fetch_min(bus as u16, Ordering::SeqCst);
}

/// Whether every bridge that could hold `bus` is in the table.
pub fn listed_through(bus: u8) -> bool {
    bus as u16 <= FIRST_LEFT_OUT.load(Ordering::SeqCst)
}

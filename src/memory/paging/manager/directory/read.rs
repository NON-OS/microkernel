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

//! Finding an address space without the paging manager lock.

use core::sync::atomic::Ordering::*;

use super::slot::{Entry, Slot};
use super::table::{FULL, HIGH, SLOTS, TABLE};

/// The entry with the lowest asid that `wanted` accepts: the one the
/// manager's own table, kept in asid order, answers first.
pub(crate) fn find(mut wanted: impl FnMut(&Entry) -> bool) -> Option<Entry> {
    let high = HIGH.load(Acquire).min(SLOTS);
    TABLE[..high].iter().filter_map(Slot::read).filter(|e| wanted(e)).min_by_key(|e| e.0)
}

/// Whether a miss in `find` proves the address space is not there: false
/// once an address space found no slot.
pub(crate) fn complete() -> bool {
    !FULL.load(Acquire)
}

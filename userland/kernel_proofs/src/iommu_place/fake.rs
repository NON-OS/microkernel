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

//! A unit for iommu::place: its tables, and apart what it sees, which
//! changes only on a flush that succeeds.

use std::cell::{Cell, RefCell};
use std::vec::Vec;

use super::place::{place, Unplaced};

/// A map of `pages` leaves. `bad_write`, `bad_clear`: a leaf whose write or
/// clear fails. `flush_fails`: flushes that fail before one succeeds, or
/// None for a unit no flush ever reaches. Returns place's answer and the
/// leaves the unit then sees.
pub(super) fn run(
    pages: u64,
    (bad_write, bad_clear): (Option<u64>, Option<u64>),
    flush_fails: Option<u32>,
) -> (Result<(), Unplaced<u8>>, Vec<bool>) {
    let tables = RefCell::new(vec![false; pages as usize]);
    let view = RefCell::new(vec![false; pages as usize]);
    let failing = Cell::new(flush_fails);
    let write = |p: u64| {
        if Some(p) == bad_write {
            return Err(1);
        }
        tables.borrow_mut()[p as usize] = true;
        Ok(())
    };
    let clear = |p: u64| {
        tables.borrow_mut()[p as usize] = false;
        Some(p) != bad_clear
    };
    let flush = || match failing.get() {
        Some(0) => {
            *view.borrow_mut() = tables.borrow().clone();
            Ok(())
        }
        Some(n) => {
            failing.set(Some(n - 1));
            Err(2)
        }
        None => Err(2),
    };
    let got = place(pages, write, clear, flush);
    (got, view.into_inner())
}

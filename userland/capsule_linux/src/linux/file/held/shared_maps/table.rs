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

//! The family's table of shared file maps, kept as the family's copies of
//! its files are (cache/table.rs): one per personality process, which is
//! one family.

use alloc::vec::Vec;
use core::cell::RefCell;

use super::entry::Entry;

struct Table(RefCell<Vec<Entry>>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Table {}

static TABLE: Table = Table(RefCell::new(Vec::new()));

/// `f` on the table. Nothing that reads a guest or a file runs inside it.
pub(super) fn with<T>(f: impl FnOnce(&mut Vec<Entry>) -> T) -> T {
    f(&mut TABLE.0.borrow_mut())
}

/// A copy of the maps `pick` takes, to work on outside the table.
pub(super) fn picked(pick: impl Fn(&Entry) -> bool) -> Vec<Entry> {
    with(|all| all.iter().filter(|e| pick(e)).cloned().collect())
}

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

//! The family's pseudo-terminals, kept where every call on them finds them.

use core::cell::RefCell;

use super::table::Table;

struct Cell(RefCell<Table>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Cell {}

static PTYS: Cell = Cell(RefCell::new(Table::new()));

pub(super) fn with<T>(f: impl FnOnce(&mut Table) -> T) -> T {
    f(&mut PTYS.0.borrow_mut())
}

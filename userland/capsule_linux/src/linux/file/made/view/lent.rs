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

/* The view the family lends /proc for one call. */

use alloc::vec::Vec;
use core::cell::RefCell;

use super::shape::View;

pub(super) struct Lent(pub(super) RefCell<View>);

/*
 * SAFETY: one personality process serves one family from one serve loop,
 * answering one call at a time, so no two borrows can overlap.
 */
unsafe impl Sync for Lent {}

static LENT: Lent = Lent(RefCell::new(View { me: 0, thread: 0, procs: Vec::new() }));

/* Lend the view for one call; an empty view takes it back. */
pub fn lend(view: View) {
    *LENT.0.borrow_mut() = view;
}

pub fn with<T>(f: impl FnOnce(&View) -> T) -> T {
    f(&LENT.0.borrow())
}

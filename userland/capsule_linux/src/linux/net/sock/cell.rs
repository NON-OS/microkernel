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

//! Where the table lives. This personality hosts one family and serves it
//! from one thread, so the table is a single process-wide value: net.sockets
//! keeps its own the same way.

use core::cell::RefCell;

use super::table::Socks;

struct One(RefCell<Socks>);

/*
 * SAFETY: the serve loop is the only thread in this capsule that reaches the
 * table; guest threads run in their own processes and only trap into it.
 */
unsafe impl Sync for One {}

static TABLE: One = One(RefCell::new(Socks::new()));

/// Run `f` on the table. A call inside `f` that reaches the table again would
/// panic on the borrow, so no function here calls out while holding it.
pub fn with<R>(f: impl FnOnce(&mut Socks) -> R) -> R {
    f(&mut TABLE.0.borrow_mut())
}

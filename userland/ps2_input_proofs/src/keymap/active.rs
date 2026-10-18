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

//! The active layout, standing in for capsule_driver_ps2_input's
//! keymap/active.rs. That file asks the policy store, a service this crate
//! has none of, so here the layout is the one the driver starts in before
//! the store ever answers, and Ctrl+Alt+Space cycles it the same way.

use std::cell::Cell;

use nonos_keymap::Layout;

thread_local! {
    static INDEX: Cell<u8> = const { Cell::new(0) };
}

pub fn current() -> Layout {
    Layout::from_index(INDEX.with(Cell::get))
}

pub fn cycle() -> Layout {
    let next = current().next();
    INDEX.with(|i| i.set(next.index()));
    next
}

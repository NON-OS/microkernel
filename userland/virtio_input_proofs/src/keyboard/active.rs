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

//! The active layout, held per test thread. The shipping file asks the
//! policy store, which a host test has none of; what the driver does with
//! the layout it gets is what is under test.

use std::cell::Cell;

use nonos_keymap::Layout;

thread_local! {
    static LAYOUT: Cell<u8> = const { Cell::new(0) };
    static CYCLES: Cell<u32> = const { Cell::new(0) };
}

pub fn current() -> Layout {
    Layout::from_index(LAYOUT.with(|l| l.get()))
}

pub fn cycle() -> Layout {
    CYCLES.with(|c| c.set(c.get() + 1));
    let next = current().next();
    LAYOUT.with(|l| l.set(next.index()));
    next
}

pub fn set_layout(layout: Layout) {
    LAYOUT.with(|l| l.set(layout.index()));
}

pub fn cycles() -> u32 {
    CYCLES.with(|c| c.get())
}

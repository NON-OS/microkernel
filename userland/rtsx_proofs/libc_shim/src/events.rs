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

//! One ordered list per test thread of what happened at the edges the driver
//! cannot see past: broker calls, and what a modelled reader was told to do.

use std::cell::RefCell;

thread_local! {
    static EVENTS: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub fn note(event: impl Into<String>) {
    EVENTS.with(|e| e.borrow_mut().push(event.into()));
}

/// Everything noted on this thread, in order.
pub fn events() -> Vec<String> {
    EVENTS.with(|e| e.borrow().clone())
}

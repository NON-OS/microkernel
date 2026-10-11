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

//! One part per test thread, reached by the driver's window.

use std::cell::RefCell;

use super::part::Part;

thread_local! {
    static PART: RefCell<Option<Part>> = const { RefCell::new(None) };
}

pub fn install(part: Part) {
    PART.with(|p| *p.borrow_mut() = Some(part));
}

/// The part, for a test to inspect after the driver is done with it.
pub fn take() -> Part {
    PART.with(|p| p.borrow_mut().take()).expect("no part installed on this thread")
}

pub fn with_part<R>(f: impl FnOnce(&mut Part) -> R) -> R {
    PART.with(|p| f(p.borrow_mut().as_mut().expect("no part installed on this thread")))
}

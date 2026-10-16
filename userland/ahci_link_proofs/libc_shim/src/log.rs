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

//! The log of broker calls, each with the device as the test's watch reads
//! it when the call is made.

use std::cell::RefCell;

use crate::calls::{Call, Engines, Seen};

type Watch = Box<dyn Fn() -> Engines>;

thread_local! {
    static SEEN: RefCell<Vec<Seen>> = const { RefCell::new(Vec::new()) };
    static WATCH: RefCell<Option<Watch>> = const { RefCell::new(None) };
}

/// Read the device through `watch` at every call from now on.
pub fn watch(watch: impl Fn() -> Engines + 'static) {
    WATCH.with(|w| *w.borrow_mut() = Some(Box::new(watch)));
}

/// Every call made on this thread, in order.
pub fn seen() -> Vec<Seen> {
    SEEN.with(|s| s.borrow().clone())
}

/// Log `call`; false when the test has the broker refuse it.
pub(crate) fn note(call: Call) -> bool {
    let device = WATCH.with(|w| w.borrow().as_ref().map(|f| f()).unwrap_or_default());
    let refused = crate::refuse::refused(call);
    SEEN.with(|s| s.borrow_mut().push(Seen { call, device, refused }));
    !refused
}

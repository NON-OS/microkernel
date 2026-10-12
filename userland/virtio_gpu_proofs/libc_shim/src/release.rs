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

//! Giving the device back: counted, and shown to the test at the moment it
//! happens, so the test can read the device then (a reset must come first).

use std::cell::{Cell, RefCell};

thread_local! {
    static RELEASES: Cell<u32> = const { Cell::new(0) };
    static WATCH: RefCell<Option<Box<dyn Fn()>>> = const { RefCell::new(None) };
}

pub fn mk_device_release(_device_id: u64) -> i64 {
    RELEASES.with(|n| n.set(n.get() + 1));
    WATCH.with(|w| {
        if let Some(watch) = w.borrow().as_ref() {
            watch();
        }
    });
    0
}

/// How many times this thread's driver gave its device back.
pub fn releases() -> u32 {
    RELEASES.with(Cell::get)
}

/// Run `watch` at each release, before it returns to the driver.
pub fn on_release(watch: impl Fn() + 'static) {
    WATCH.with(|w| *w.borrow_mut() = Some(Box::new(watch)));
}

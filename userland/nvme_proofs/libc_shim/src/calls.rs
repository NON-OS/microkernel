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

//! The broker calls the driver made on this thread, in order, so a test can
//! see what was handed back and when, and the calls a test has the broker
//! refuse.

use std::cell::{Cell, RefCell};

thread_local! {
    static CALLS: RefCell<Vec<(&'static str, u64)>> = const { RefCell::new(Vec::new()) };
    static REFUSED: Cell<(&'static str, Option<u64>)> = const { Cell::new(("", None)) };
}

/// Have the broker refuse every later `call` on this thread, and no other
/// ("" for none).
pub fn refuse(call: &'static str) {
    REFUSED.with(|r| r.set((call, None)));
}

/// Have the broker refuse every later `call` made with `arg`, and no other.
pub fn refuse_only(call: &'static str, arg: u64) {
    REFUSED.with(|r| r.set((call, Some(arg))));
}

/// Note `call` and its argument: 0, or -1 when the test refuses it.
pub(crate) fn answer(call: &'static str, arg: u64) -> i64 {
    record(call, arg);
    let (refused, only) = REFUSED.with(Cell::get);
    if refused == call && only.is_none_or(|a| a == arg) {
        -1
    } else {
        0
    }
}

/// Note one call and its argument.
pub fn record(call: &'static str, arg: u64) -> i64 {
    CALLS.with(|c| c.borrow_mut().push((call, arg)));
    0
}

/// Every call recorded on this thread, as (call, argument), in order.
pub fn calls() -> Vec<(&'static str, u64)> {
    CALLS.with(|c| c.borrow().clone())
}

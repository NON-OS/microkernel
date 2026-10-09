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

//! The console, kept per test thread. Every "igc:" line the driver sends is
//! recorded whole, so a test can hold the owner's photo to its exact text.

use std::cell::RefCell;

thread_local! {
    static LINES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    if buf.is_null() || len == 0 {
        return -22;
    }
    /*
     * SAFETY: the driver hands a buffer of `len` readable bytes, as the real
     * call requires.
     */
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) };
    let line = String::from_utf8_lossy(bytes).into_owned();
    LINES.with(|l| l.borrow_mut().push(line));
    len as i64
}

/// Every line sent on this thread, in order, each with its newline.
pub fn said() -> Vec<String> {
    LINES.with(|l| l.borrow().clone())
}

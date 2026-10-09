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

//! The console. Every line the driver writes on this thread is kept, so a
//! test can hold the driver to the exact text the owner is asked to read.

use std::cell::RefCell;

thread_local! {
    static LINES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    if buf.is_null() || len == 0 {
        return -22;
    }
    /*
     * SAFETY: the driver hands a pointer to `len` initialised bytes of its
     * own line buffer, as the real call requires.
     */
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) };
    let line = String::from_utf8_lossy(bytes).trim_end_matches('\n').to_string();
    LINES.with(|l| l.borrow_mut().push(line));
    len as i64
}

/// Every line written on this thread, oldest first.
pub fn logged() -> Vec<String> {
    LINES.with(|l| l.borrow().clone())
}

pub fn clear_log() {
    LINES.with(|l| l.borrow_mut().clear());
}

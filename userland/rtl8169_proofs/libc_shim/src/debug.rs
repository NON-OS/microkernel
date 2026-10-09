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

//! The console line call, kept so a test can read what the driver logged.

thread_local! {
    static LOGGED: std::cell::RefCell<Vec<String>> = const { std::cell::RefCell::new(Vec::new()) };
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    if buf.is_null() || len == 0 {
        return -22;
    }
    // SAFETY: the caller hands `len` readable bytes, as the real call requires.
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) };
    let line = String::from_utf8_lossy(bytes).into_owned();
    LOGGED.with(|l| l.borrow_mut().push(line));
    0
}

/// Every line logged on this thread, in order, each with its newline.
pub fn logged() -> Vec<String> {
    LOGGED.with(|l| l.borrow().clone())
}

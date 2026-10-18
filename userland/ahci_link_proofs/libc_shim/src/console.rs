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

//! The serial console, kept for the test to read.

use std::cell::RefCell;

thread_local! {
    static SAID: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    // SAFETY: the driver passes a line of its own `len` bytes long.
    let line = unsafe { std::slice::from_raw_parts(buf, len) };
    let text = String::from_utf8_lossy(line).trim_end().to_owned();
    SAID.with(|s| s.borrow_mut().push(text));
    0
}

/// Every console line this thread's driver wrote.
pub fn said() -> Vec<String> {
    SAID.with(|s| s.borrow().clone())
}

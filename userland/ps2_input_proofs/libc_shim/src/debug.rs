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

//! The console lines the driver writes, kept for the test to read back.

use std::cell::RefCell;

thread_local! {
    static DEBUG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_debug(msg: *const u8, len: usize) -> i64 {
    let line = String::from_utf8_lossy(bytes(msg, len)).into_owned();
    DEBUG.with(|d| d.borrow_mut().push(line));
    0
}

/// The lines the driver wrote, oldest first, leaving the record empty.
pub fn take_debug() -> Vec<String> {
    DEBUG.with(|d| std::mem::take(&mut *d.borrow_mut()))
}

fn bytes<'a>(msg: *const u8, len: usize) -> &'a [u8] {
    /*
     * SAFETY: the driver passes its own buffer and the length it filled,
     * live for the call.
     */
    unsafe { std::slice::from_raw_parts(msg, len) }
}

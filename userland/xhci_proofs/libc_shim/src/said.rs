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

//! What the driver said on the debug console, kept per test thread so a
//! test can hold it to saying a thing once.

use std::cell::RefCell;

thread_local! {
    static SAID: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    if buf.is_null() {
        return -14;
    }
    // SAFETY: the driver passes a live string's pointer and length, as the
    // kernel's debug call requires of it.
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) };
    SAID.with(|s| s.borrow_mut().push(String::from_utf8_lossy(bytes).into_owned()));
    0
}

/// Every line this thread said so far, and the record cleared.
pub fn take_said() -> Vec<String> {
    SAID.with(|s| std::mem::take(&mut *s.borrow_mut()))
}

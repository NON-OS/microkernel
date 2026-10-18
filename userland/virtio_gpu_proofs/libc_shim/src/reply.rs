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

//! Replies, kept for the test to read.

use std::cell::RefCell;

thread_local! {
    static REPLIES: RefCell<Vec<(u32, Vec<u8>)>> = const { RefCell::new(Vec::new()) };
}

/// Keep the reply `len` bytes at `buf` for `pid`.
pub fn mk_ipc_reply(pid: u32, buf: *const u8, len: usize) -> i64 {
    // SAFETY: the driver passes its own transmit buffer and a length inside
    // it, as it does to the kernel.
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) }.to_vec();
    REPLIES.with(|r| r.borrow_mut().push((pid, bytes)));
    0
}

/// Every reply this thread's driver made, oldest first.
pub fn replies() -> Vec<(u32, Vec<u8>)> {
    REPLIES.with(|r| r.borrow().clone())
}

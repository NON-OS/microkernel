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
//! Replies, kept for the test to read: the last one on this thread.

use std::cell::RefCell;

thread_local! {
    static LAST: RefCell<Option<(u32, Vec<u8>)>> = const { RefCell::new(None) };
}

pub fn mk_ipc_reply(dest_pid: u32, buf: *const u8, len: usize) -> i64 {
    // SAFETY: the server hands its own transmit buffer, `len` bytes long.
    let bytes = unsafe { core::slice::from_raw_parts(buf, len) }.to_vec();
    LAST.with(|l| *l.borrow_mut() = Some((dest_pid, bytes)));
    len as i64
}

/// The last reply and who it went to, taken.
pub fn take_reply() -> Option<(u32, Vec<u8>)> {
    LAST.with(|l| l.borrow_mut().take())
}

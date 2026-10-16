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

//! The server's IPC, run by a test instead of a kernel: nothing arrives on
//! its own, and every reply the driver sends, to a capsule or to the
//! kernel's fixed endpoint, is kept on the sending thread for the test.

use std::cell::RefCell;

thread_local! {
    static REPLIES: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

/// The replies sent on this thread since the last call, oldest first.
pub fn take_replies() -> Vec<Vec<u8>> {
    REPLIES.with(|r| core::mem::take(&mut *r.borrow_mut()))
}

fn keep(buf: *const u8, len: usize) -> i64 {
    // SAFETY: the driver passes its own reply buffer of at least `len` bytes.
    let bytes = unsafe { core::slice::from_raw_parts(buf, len) };
    REPLIES.with(|r| r.borrow_mut().push(bytes.to_vec()));
    len as i64
}

pub fn mk_ipc_reply(_dest_pid: u32, buf: *const u8, len: usize) -> i64 {
    keep(buf, len)
}

pub fn mk_ipc_send(_endpoint: u64, buf: *const u8, len: usize) -> i64 {
    keep(buf, len)
}

/// Nothing is ever waiting: requests reach the server through `dispatch`.
pub fn mk_ipc_recv_from(_ep: u64, _buf: *mut u8, _len: usize, _t: u64, _pid: *mut u32) -> i64 {
    -11
}

pub fn recv_ready(rc: i64) -> bool {
    rc >= 0
}

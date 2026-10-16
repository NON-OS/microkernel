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

//! The IPC the block service receives on. Requests a test queues arrive in
//! order with the pid the kernel stamps; with none queued a receive waits
//! out its timeout on the virtual clock. Replies are in `replies.rs`.

use std::cell::RefCell;
use std::collections::VecDeque;

thread_local! {
    static INBOX: RefCell<VecDeque<(u32, Vec<u8>)>> = const { RefCell::new(VecDeque::new()) };
}

/// Queue a request from `sender`.
pub fn push_request(sender: u32, bytes: Vec<u8>) {
    INBOX.with(|i| i.borrow_mut().push_back((sender, bytes)));
}

pub fn mk_ipc_recv_from(_ep: u64, buf: *mut u8, len: usize, ms: u64, from: &mut u32) -> i64 {
    let Some((sender, bytes)) = INBOX.with(|i| i.borrow_mut().pop_front()) else {
        crate::clock::advance_ms(ms);
        return -110;
    };
    let n = bytes.len().min(len);
    // SAFETY: the caller hands `len` writable bytes, as the real call
    // requires, and `n` is at most `len`.
    unsafe { std::ptr::copy_nonoverlapping(bytes.as_ptr(), buf, n) };
    *from = sender;
    n as i64
}

pub fn recv_ready(rc: i64) -> bool {
    rc > 0
}

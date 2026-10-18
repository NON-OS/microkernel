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

//! The inbox, per thread: the messages a test sends, how long each receive
//! was asked to wait, and every reply. A receive with nothing queued has
//! waited its time out.

use std::cell::RefCell;
use std::collections::VecDeque;

/// What the kernel answers a receive whose wait ran out.
pub const ERRNO_TIMEDOUT: i64 = -110;

thread_local! {
    static INBOX: RefCell<VecDeque<Result<(u32, Vec<u8>), i64>>> =
        const { RefCell::new(VecDeque::new()) };
    static WAITS: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
    static REPLIES: RefCell<Vec<(u32, Vec<u8>)>> = const { RefCell::new(Vec::new()) };
}

/// The next receive gets `msg` from `sender`, or fails at once with `rc`.
pub fn send(next: Result<(u32, &[u8]), i64>) {
    INBOX.with(|i| i.borrow_mut().push_back(next.map(|(pid, msg)| (pid, msg.to_vec()))));
}

pub fn mk_ipc_recv_from(_ep: u64, buf: *mut u8, len: usize, wait: u64, from: *mut u32) -> i64 {
    WAITS.with(|w| w.borrow_mut().push(wait));
    let (sender, msg) = match INBOX.with(|i| i.borrow_mut().pop_front()) {
        None => return ERRNO_TIMEDOUT,
        Some(Err(rc)) => return rc,
        Some(Ok(m)) => m,
    };
    let n = msg.len().min(len);
    // SAFETY: the caller hands a buffer of `len` bytes and a sender slot,
    // as the real call requires; at most `len` bytes are copied, as the
    // kernel cuts a longer message.
    unsafe {
        std::ptr::copy_nonoverlapping(msg.as_ptr(), buf, n);
        *from = sender;
    }
    n as i64
}

pub fn mk_ipc_reply(dest_pid: u32, buf: *const u8, len: usize) -> i64 {
    // SAFETY: the caller hands `len` bytes at `buf`, as the real call does.
    let reply = unsafe { std::slice::from_raw_parts(buf, len) }.to_vec();
    REPLIES.with(|r| r.borrow_mut().push((dest_pid, reply)));
    0
}

/// How long each receive on this thread was asked to wait, in order.
pub fn waits() -> Vec<u64> {
    WAITS.with(|w| w.borrow().clone())
}

/// Every reply on this thread, as (pid, bytes), in order.
pub fn replies() -> Vec<(u32, Vec<u8>)> {
    REPLIES.with(|r| r.borrow().clone())
}

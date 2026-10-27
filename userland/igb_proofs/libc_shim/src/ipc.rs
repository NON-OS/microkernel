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

//! The IPC calls a server loop makes: requests a test lines up, answered in
//! order by `mk_ipc_recv_from`, and every reply kept. When the requests run
//! out the receive panics with `SERVED`, which is how a test ends a loop
//! that never returns and reads back what it answered.

use std::cell::RefCell;

pub const SERVED: &str = "every lined-up request served";

type Messages = RefCell<Vec<(u32, Vec<u8>)>>;

thread_local! {
    static INBOX: Messages = const { RefCell::new(Vec::new()) };
    static REPLIES: Messages = const { RefCell::new(Vec::new()) };
}

/// The requests the next receives answer with, each as (sender, bytes).
pub fn line_up_requests(requests: &[(u32, Vec<u8>)]) {
    INBOX.with(|i| *i.borrow_mut() = requests.iter().rev().cloned().collect());
    REPLIES.with(|r| r.borrow_mut().clear());
}

/// Every reply sent since the requests were lined up, as (to, bytes).
pub fn replies() -> Vec<(u32, Vec<u8>)> {
    REPLIES.with(|r| r.borrow().clone())
}

pub fn mk_ipc_recv_from(_ep: u64, buf: *mut u8, len: usize, _ms: u64, from: *mut u32) -> i64 {
    let Some((sender, msg)) = INBOX.with(|i| i.borrow_mut().pop()) else { panic!("{SERVED}") };
    let n = msg.len().min(len);
    // SAFETY: the caller hands `len` writable bytes and a live u32, as the
    // real call requires; `n` is at most `len`.
    unsafe {
        std::ptr::copy_nonoverlapping(msg.as_ptr(), buf, n);
        *from = sender;
    }
    n as i64
}

pub fn mk_ipc_reply(to: u32, buf: *const u8, len: usize) -> i64 {
    // SAFETY: the caller hands `len` readable bytes, as the real call requires.
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) }.to_vec();
    REPLIES.with(|r| r.borrow_mut().push((to, bytes)));
    0
}

/// As nonos_libc's: a receive that brought a message is served.
pub fn recv_ready(rc: i64) -> bool {
    rc > 0
}

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

//! The reply path, the StoreWrite answer, the console and the clock.

use std::cell::{Cell, RefCell};
use std::time::{Duration, Instant};

thread_local! {
    static SENT: RefCell<Vec<(u64, Vec<u8>)>> = const { RefCell::new(Vec::new()) };
    static LOG: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    static STORE_WRITE: Cell<bool> = const { Cell::new(false) };
    static CAP_ASKED: RefCell<Vec<(u32, u64)>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_ipc_send(endpoint: u64, buf: *const u8, len: usize) -> i64 {
    // SAFETY: the caller hands `len` readable bytes, as the real call requires.
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) }.to_vec();
    SENT.with(|r| r.borrow_mut().push((endpoint, bytes)));
    0
}

/// Every message sent on this thread since the last call, as (endpoint,
/// bytes).
pub fn sent() -> Vec<(u64, Vec<u8>)> {
    SENT.with(|r| std::mem::take(&mut *r.borrow_mut()))
}

/// Whether the kernel says a sender holds StoreWrite, for this thread.
pub fn store_write(holds: bool) {
    STORE_WRITE.with(|s| s.set(holds));
}

pub fn mk_cap_check(pid: u32, mask: u64) -> bool {
    CAP_ASKED.with(|c| c.borrow_mut().push((pid, mask)));
    STORE_WRITE.with(Cell::get)
}

pub fn mk_getpid() -> u32 {
    77
}

/// The serve loop is not run on the host: nothing ever arrives.
pub fn mk_ipc_recv_from(_e: u64, _b: *mut u8, _l: usize, _t: u64, _s: *mut u32) -> i64 {
    -11
}

pub fn recv_ready(rc: i64) -> bool {
    rc >= 0
}

pub fn mk_debug(buf: *const u8, len: usize) -> i64 {
    // SAFETY: the caller hands `len` readable bytes, as the real call requires.
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) };
    LOG.with(|l| l.borrow_mut().push(String::from_utf8_lossy(bytes).into_owned()));
    len as i64
}

/// Every console line on this thread.
pub fn logged() -> Vec<String> {
    LOG.with(|l| l.borrow().clone())
}

pub fn mk_idle_ms(ms: u64) -> i64 {
    std::thread::sleep(Duration::from_millis(ms));
    0
}

pub fn mk_yield() -> i64 {
    std::thread::yield_now();
    0
}

pub struct Deadline {
    end: Instant,
}

impl Deadline {
    pub fn after_ms(ms: u64) -> Self {
        Self { end: Instant::now() + Duration::from_millis(ms) }
    }
    pub fn expired(&self) -> bool {
        Instant::now() >= self.end
    }
}

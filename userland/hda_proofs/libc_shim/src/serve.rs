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

//! What the serving loop sends and waits on: every reply the handlers send,
//! kept per thread for a test to read, and an interrupt that never comes,
//! so the loop's completion check runs on what the registers say.

use std::cell::RefCell;

use crate::broker::record;

pub const MK_IRQ_WAIT_TIMED_OUT: i64 = 1;

thread_local! {
    static SENT: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_ipc_send(_endpoint: u64, buf: *const u8, len: usize) -> i64 {
    // SAFETY: the caller hands `len` readable bytes, as the real call requires.
    let bytes = unsafe { std::slice::from_raw_parts(buf, len) };
    SENT.with(|s| s.borrow_mut().push(bytes.to_vec()));
    0
}

/// Every reply sent on this thread, in order.
pub fn replies() -> Vec<Vec<u8>> {
    SENT.with(|s| s.borrow().clone())
}

pub fn mk_irq_wait(grant_id: u64, _last: u64, _ms: u64, _seq: *mut u64) -> i64 {
    record("irq_wait", grant_id);
    MK_IRQ_WAIT_TIMED_OUT
}

pub fn mk_irq_ack(grant_id: u64) -> i64 {
    record("irq_ack", grant_id)
}

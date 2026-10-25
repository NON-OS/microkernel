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

//! The sleep, the clock and the log line the shared policy calls, per
//! thread: each sleep is recorded, so a test sees a loop that sleeps where
//! it should have waited, the clock moves only by them, and the log is
//! dropped.

use std::cell::RefCell;

thread_local! {
    static IDLES: RefCell<Vec<u64>> = const { RefCell::new(Vec::new()) };
}

pub fn mk_idle_ms(ms: u64) -> i64 {
    IDLES.with(|i| i.borrow_mut().push(ms));
    0
}

/// Every sleep on this thread, in milliseconds, in order.
pub fn idles() -> Vec<u64> {
    IDLES.with(|i| i.borrow().clone())
}

/// Milliseconds since boot: what the sleeps on this thread took, the only
/// time that passes in a test.
pub fn mk_uptime_ms() -> i64 {
    IDLES.with(|i| i.borrow().iter().sum::<u64>() as i64)
}

pub fn mk_debug(_buf: *const u8, len: usize) -> i64 {
    len as i64
}

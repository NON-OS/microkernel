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

//! The learned answer time of each receive op, and the timed call that
//! keeps it (learned_bound.rs has the rule).

use core::sync::atomic::{AtomicU64, Ordering};

use super::learned_bound::{bound, learn, sample, OpBound};

/// Learned answer times; zero starts each op at its floor.
pub static BATCH_MS: AtomicU64 = AtomicU64::new(0);
pub static SINGLE_MS: AtomicU64 = AtomicU64::new(0);

/// Calls and timeouts of the op whose learned time `slot` is, both packed
/// in one word (calls high, timeouts low), for the summary line.
fn tally(slot: &AtomicU64) -> &'static AtomicU64 {
    static BATCH: AtomicU64 = AtomicU64::new(0);
    static SINGLE: AtomicU64 = AtomicU64::new(0);
    if core::ptr::eq(slot, &BATCH_MS) {
        &BATCH
    } else {
        &SINGLE
    }
}

/// Make `call` with the op's learned bound, reading `now` before and after,
/// and learn from it. A negative answer is no answer.
pub fn timed_with(
    slot: &AtomicU64,
    op: &OpBound,
    now: impl Fn() -> i64,
    call: impl FnOnce(u64) -> i64,
) -> i64 {
    let ewma = slot.load(Ordering::Relaxed);
    let waited = bound(op, ewma);
    let start = now();
    let n = call(waited);
    let elapsed = now().saturating_sub(start).max(0) as u64;
    slot.store(learn(ewma, sample(n >= 0, elapsed, waited)), Ordering::Relaxed);
    let t = tally(slot).fetch_add((1 << 32) | u64::from(n < 0), Ordering::Relaxed) + (1 << 32);
    super::rx_summary::at(
        core::ptr::eq(slot, &BATCH_MS),
        t >> 32,
        t & 0xffff_ffff,
        bound(op, ewma),
    );
    n
}

/// `timed_with` on the capsule's clock.
pub fn timed(slot: &AtomicU64, op: &OpBound, call: impl FnOnce(u64) -> i64) -> i64 {
    timed_with(slot, op, clock, call)
}

/// The capsule's clock as a plain fn (libc's is `extern "C"`).
fn clock() -> i64 {
    nonos_libc::mk_time_millis()
}

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

//! Where the serve loop is, in atomics a second thread can read while the
//! loop itself is stuck (`stall_watch.rs`): the turn, the step, since when,
//! and the request being served.

use core::sync::atomic::{AtomicI64, AtomicU32, AtomicU64, AtomicU8, Ordering};

use super::stall_step::{Seen, Step};

static TURN: AtomicU32 = AtomicU32::new(0);
static STEP: AtomicU8 = AtomicU8::new(Step::Wait as u8);
static SINCE: AtomicI64 = AtomicI64::new(0);
/// The request in hand: op in the high 32 bits, the sender's pid below.
static REQUEST: AtomicU64 = AtomicU64::new(0);

/// A new turn begins.
pub fn turn(now: i64) {
    TURN.fetch_add(1, Ordering::AcqRel);
    step(Step::Wait, now);
}

/// The loop moves to `s` at `now`.
pub fn step(s: Step, now: i64) {
    SINCE.store(now, Ordering::Release);
    STEP.store(s as u8, Ordering::Release);
}

/// The loop takes up a request.
pub fn request(op: u16, from: u32, now: i64) {
    REQUEST.store((op as u64) << 32 | from as u64, Ordering::Release);
    step(Step::Request, now);
}

/// The loop's place as of now.
pub fn read() -> Seen {
    let r = REQUEST.load(Ordering::Acquire);
    Seen {
        turn: TURN.load(Ordering::Acquire),
        step: Step::from(STEP.load(Ordering::Acquire)),
        since: SINCE.load(Ordering::Acquire),
        op: (r >> 32) as u16,
        from: r as u32,
    }
}

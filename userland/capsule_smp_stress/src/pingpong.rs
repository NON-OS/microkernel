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

//! Futex hand-offs. Two threads share a word and take turns moving it: side 0
//! from even to odd, side 1 back, each waking the other after its move. A
//! turn that comes back a second late had its wake lost, since the partner
//! moves the moment it runs.

use crate::clock::now_ms;
use crate::stats::{self, Kind};
use crate::{futex, stall};
use core::sync::atomic::{AtomicU32, Ordering};

pub const PAIRS: usize = 4;
static WORDS: [AtomicU32; PAIRS] = [const { AtomicU32::new(0) }; PAIRS];

/// `arg` is pair * 2 + side.
pub fn run(arg: usize) {
    let word = &WORDS[arg / 2];
    let side = (arg % 2) as u32;
    let mut handed_at = now_ms();
    while !stats::stopping() {
        let seen = word.load(Ordering::Acquire);
        if seen % 2 == side {
            let waited = now_ms().saturating_sub(handed_at);
            stats::round(Kind::Futex, waited);
            if stall::is_stall(waited) {
                stats::fault(Kind::Stall);
            }
            word.store(seen.wrapping_add(1), Ordering::Release);
            handed_at = now_ms();
            if futex::wake(word, 1) < 0 {
                stats::fault(Kind::Error);
            }
            continue;
        }
        // A negative answer is a refusal, not a timeout: a timed-out wait
        // returns 0 like a woken one, and the loop looks at the word again.
        if futex::wait(word, seen, stall::WAIT_TIMEOUT_MS) < 0 {
            stats::fault(Kind::Error);
        }
    }
    // The partner may be waiting on this side's move; it times out and sees
    // the stop on its own, so no wake is owed here.
}

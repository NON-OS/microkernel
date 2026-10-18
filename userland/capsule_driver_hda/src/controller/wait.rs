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

//! A bounded wait on a register, in milliseconds of uptime: the HD Audio
//! specification states its limits in time (521 us for a codec to signal
//! after CRST, 100 ms Linux allows a reset bit to move), and a spin count is
//! no time at all. A working part answers inside a millisecond, and a wait
//! spins for at least one whole one; past that each poll sleeps, so a part
//! that never answers holds no core. The poll count bounds a wait only on a clock that is stuck.

use crate::clock::{now_ms, pause_ms};

/// Far more polls than any wait here makes inside its time limit.
const POLL_CAP: u32 = 50_000_000;
/// Spin while the clock reads under this: two is at least a whole
/// millisecond, where one ends microseconds in on a wait begun late in one.
const SPIN_MS: u64 = 2;

pub struct Wait {
    start: u64,
    ms: u64,
    polls: u32,
}

impl Wait {
    pub fn ms(ms: u64) -> Self {
        Wait { start: now_ms(), ms, polls: 0 }
    }

    /// Counts one poll and says whether the wait is over.
    pub fn expired(&mut self) -> bool {
        self.polls = self.polls.saturating_add(1);
        if self.polls >= POLL_CAP {
            return true;
        }
        // Strictly past: with a millisecond clock, a wait begun late in one
        // tick would otherwise end at the next, well short of `ms`.
        now_ms().saturating_sub(self.start) > self.ms
    }

    /// Between two polls: a spin for `SPIN_MS` of the clock, a sleep after.
    pub fn idle(&self) {
        if now_ms().saturating_sub(self.start) < SPIN_MS {
            return core::hint::spin_loop();
        }
        pause_ms(1);
    }
}

/// Poll `done` until it holds or `ms` milliseconds pass. True if it held.
pub fn until(ms: u64, mut done: impl FnMut() -> bool) -> bool {
    let mut w = Wait::ms(ms);
    loop {
        if done() {
            return true;
        }
        if w.expired() {
            return done();
        }
        w.idle();
    }
}

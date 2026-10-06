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

//! A bounded wait on a register, measured in milliseconds of uptime.
//!
//! Every handshake in this driver used to count spins, and a spin count is no
//! time at all: a million reads of a register takes a few milliseconds on one
//! machine and a second on another, and the HD Audio specification states its
//! limits in time (521 us for a codec to signal after CRST, 100 ms Linux allows
//! a reset bit to move). The poll count is kept as a second bound only so that
//! a clock that cannot be read still ends the wait.

use crate::clock::now_ms;

/// Far more polls than any of these waits needs at its time limit, so it
/// bounds a wait only when the clock does not move.
const POLL_CAP: u32 = 50_000_000;

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
        core::hint::spin_loop();
    }
}

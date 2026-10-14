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

//! How a wait on the controller spends its time (plan/50 C7). It is bounded
//! by elapsed time, never by a count of polls. A byte takes 23 us at 400 kHz
//! and a yield on a busy machine can cost a whole time slice, so while the
//! bus moves, and for about a FIFO's drain time after, the wait polls back
//! to back. Past that it yields between polls and reads the clock after
//! every yield, so a slice runs past the deadline once at most. Once nothing
//! has moved for BRISK_MS, a device is stretching the clock or the bus has
//! stopped, and the wait sleeps a millisecond between polls instead.

use nonos_libc::{mk_idle_ms, mk_uptime_ms, mk_yield};

/// About a full FIFO's drain time: 64 bytes at 400 kHz take 1.4 ms, and the
/// clock counts whole milliseconds.
const SPIN_MS: u64 = 2;
/// How long after the last sign of progress the wait yields, not sleeps.
const BRISK_MS: u64 = 4;
/// Back-to-back polls between two reads of the clock, which is a system call.
const CLOCK_EVERY: u32 = 16;

pub struct Pace {
    end_ms: u64,
    now_ms: u64,
    quiet_since_ms: u64,
    polls: u32,
}

impl Pace {
    pub fn new(budget_ms: u64) -> Self {
        let now_ms = now();
        Pace { end_ms: now_ms.saturating_add(budget_ms), now_ms, quiet_since_ms: now_ms, polls: 0 }
    }

    /// Something moved: the wait polls back to back again.
    pub fn moved(&mut self) {
        self.quiet_since_ms = self.now_ms;
    }

    /// Wait before the next poll; false once the budget is spent.
    pub fn wait(&mut self) -> bool {
        let quiet_ms = self.now_ms.saturating_sub(self.quiet_since_ms);
        if quiet_ms < SPIN_MS {
            self.polls = self.polls.wrapping_add(1);
            if !self.polls.is_multiple_of(CLOCK_EVERY) {
                core::hint::spin_loop();
                return true;
            }
        } else if quiet_ms < BRISK_MS {
            let _ = mk_yield();
        } else {
            let _ = mk_idle_ms(1);
        }
        self.now_ms = now();
        self.now_ms < self.end_ms
    }
}

fn now() -> u64 {
    mk_uptime_ms().max(0) as u64
}

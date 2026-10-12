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

//! Spacing the resets of a device that keeps stalling. A
//! backend that holds every transmit buffer with the link up would
//! otherwise be reset, released, claimed and logged every two seconds for
//! as long as it holds them. A stall after a completed frame is reset at
//! once; each stall again with nothing completed since the last reset
//! waits twice as long as the one before, up to the cap, measured by the
//! clock. Only a longer wait is logged, so the log names each step once.

use crate::constants::{RESET_BACKOFF_MAX_MS, RESET_BACKOFF_MS};

/// What to do with a device found stalled.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Not yet: the wait since the last reset is not over.
    Wait,
    /// Reset it now; `log` when this reset starts a longer wait.
    Reset { log: bool },
}

#[derive(Default)]
pub struct Backoff {
    /// The wait the last reset set; zero before the first.
    wait_ms: u64,
    /// No reset before this uptime.
    not_before: u64,
}

impl Backoff {
    /// The step for a device found stalled at `now_ms`. `completed`: it
    /// gave a buffer back since the last reset, so this stall is a new one.
    pub fn step(&mut self, now_ms: u64, completed: bool) -> Step {
        if completed {
            *self = Self::default();
        }
        if now_ms < self.not_before {
            return Step::Wait;
        }
        let wait = self.wait_ms.saturating_mul(2).clamp(RESET_BACKOFF_MS, RESET_BACKOFF_MAX_MS);
        let log = wait != self.wait_ms;
        self.wait_ms = wait;
        self.not_before = now_ms.saturating_add(wait);
        Step::Reset { log }
    }
}

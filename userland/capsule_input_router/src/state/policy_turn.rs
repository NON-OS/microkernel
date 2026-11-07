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

//! When the router reads the settings the policy store holds: the pointer's
//! speed, the screen blank and the auto-lock. Each read is a call inside
//! the router's loop that waits up to its timeout for the store, and all of
//! them in one pass held input that came meanwhile for as long as four
//! timeouts. Now one step is read a pass, only in a pass that routed no
//! input, and a round in which the store does not answer ends there until
//! the next. Pure, so the host proofs hold it.

/// How often the settings are read again.
pub const ROUND_MS: i64 = 2000;

/// One read of a round.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    Sensitivity,
    ScreenTimeout,
    /// The auto-lock time and whether an idle inhibit holds it back.
    AutoLock,
}

const STEPS: [Step; 3] = [Step::Sensitivity, Step::ScreenTimeout, Step::AutoLock];

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct PolicyTurn {
    next: usize,
    due_ms: i64,
}

impl PolicyTurn {
    pub const fn new() -> PolicyTurn {
        PolicyTurn { next: 0, due_ms: 0 }
    }

    /// The step to read at `now`, in a pass that `routed` input or not.
    pub fn take(&self, now: i64, routed: bool) -> Option<Step> {
        if routed || now < self.due_ms {
            return None;
        }
        STEPS.get(self.next).copied()
    }

    /// The step taken was read at `now`, and the store `answered` or not.
    /// The round ends after its last step, or at once when the store did
    /// not answer.
    pub fn read(&mut self, now: i64, answered: bool) {
        self.next = self.next.saturating_add(1);
        if !answered || self.next >= STEPS.len() {
            self.next = 0;
            self.due_ms = now.saturating_add(ROUND_MS);
        }
    }
}

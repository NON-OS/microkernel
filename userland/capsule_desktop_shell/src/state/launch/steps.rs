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

//! What each turn does with a launch: wait, probe, or settle it.

use super::follow::Launch;
use super::timing::{Step, DEADLINE_MS, PROBE_GAP_MAX_MS};
use crate::state::Uptime;

/// The installer kept no outcome for the load (its ENOENT).
const NO_OUTCOME: i32 = -2;

impl Launch {
    /// The next step, given the app's pid if it has appeared.
    pub fn step(&self, now: Uptime, found: Option<u32>) -> Step {
        if let Some(pid) = found {
            return Step::Started(pid);
        }
        match self.served {
            Some(status) if status < 0 && status != NO_OUTCOME => return Step::Refused(status),
            Some(_) => return Step::Failed,
            None if self.elapsed_ms(now) >= DEADLINE_MS => return Step::Failed,
            None => {}
        }
        if now.0 >= self.next_probe_ms {
            return Step::Probe;
        }
        Step::Wait
    }

    /// What a probe found. Answered, the load is over and its status
    /// settles the next step; unanswered, the next probe waits twice as long.
    pub fn probed(&mut self, now: Uptime, answer: Option<i32>) {
        let now_ms = now.0;
        if answer.is_some() {
            self.served = answer;
            return;
        }
        self.probe_gap_ms = self.probe_gap_ms.saturating_mul(2).min(PROBE_GAP_MAX_MS);
        self.next_probe_ms = now_ms.saturating_add(self.probe_gap_ms);
    }

    /// Whether the installer is known to have finished the load.
    pub fn served(&self) -> bool {
        self.served.is_some()
    }

    /// How long the launch has run, which bounds the age of a child it made.
    pub fn elapsed_ms(&self, now: Uptime) -> i64 {
        now.0.saturating_sub(self.started_ms)
    }
}

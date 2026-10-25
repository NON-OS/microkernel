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

impl Launch {
    /// The next step, given the app's pid if it has appeared.
    pub fn step(&self, now: Uptime, found: Option<u32>) -> Step {
        if let Some(pid) = found {
            return Step::Started(pid);
        }
        if self.served || self.elapsed_ms(now) >= DEADLINE_MS {
            return Step::Failed;
        }
        if now.0 >= self.next_probe_ms {
            return Step::Probe;
        }
        Step::Wait
    }

    /// What a probe found. Answered, the load is over and the next step
    /// settles it; unanswered, the next probe waits twice as long.
    pub fn probed(&mut self, now: Uptime, answered: bool) {
        let now_ms = now.0;
        if answered {
            self.served = true;
            return;
        }
        self.probe_gap_ms = self.probe_gap_ms.saturating_mul(2).min(PROBE_GAP_MAX_MS);
        self.next_probe_ms = now_ms.saturating_add(self.probe_gap_ms);
    }

    /// Whether the installer is known to have finished the load.
    pub fn served(&self) -> bool {
        self.served
    }

    /// How long the launch has run, which bounds the age of a child it made.
    pub fn elapsed_ms(&self, now: Uptime) -> i64 {
        now.0.saturating_sub(self.started_ms)
    }
}

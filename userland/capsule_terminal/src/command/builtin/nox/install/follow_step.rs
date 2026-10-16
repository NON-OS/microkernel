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

//! What each tick of a follow decides, and what a probe of the installer found.

use super::{Follow, Step, DEADLINE_MS, PROBE_GAP_MAX_MS};

impl Follow {
    /// The next step, given the new child if one has appeared.
    pub fn step(&self, now_ms: i64, found: Option<u32>) -> Step {
        if let Some(pid) = found {
            return Step::Loaded(pid);
        }
        if self.served {
            return Step::Failed;
        }
        if self.elapsed_ms(now_ms) >= DEADLINE_MS {
            return Step::TimedOut;
        }
        if now_ms >= self.next_probe_ms {
            return Step::Probe;
        }
        Step::Wait
    }

    /// What a probe found. Answered, the load is over and the next step
    /// settles it; unanswered, the next probe waits twice as long.
    pub fn probed(&mut self, now_ms: i64, answered: bool) {
        if answered {
            self.served = true;
            return;
        }
        self.probe_gap_ms = self.probe_gap_ms.saturating_mul(2).min(PROBE_GAP_MAX_MS);
        self.next_probe_ms = now_ms.saturating_add(self.probe_gap_ms);
    }
}

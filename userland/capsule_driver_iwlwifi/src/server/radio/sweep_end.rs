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

//! What a step of the sweep came to.

use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::sweep::Tick;

use super::say::Line;
use super::state::{Radio, REST_MS, STALLS_SAID};

impl Radio {
    // What a step of the sweep came to: the scan list aged and the next sweep
    // after a rest, or the firmware restarted.
    pub(super) fn after_step(&mut self, tick: Tick, now: u64) {
        match tick {
            Tick::Completed(_) => {
                if self.sweep.completed == 1 {
                    Line::new()
                        .text(b"first scan complete: ")
                        .num(self.results.count() as u32)
                        .text(b" networks from ")
                        .num(self.beacons)
                        .text(b" beacons")
                        .send();
                }
                self.results.end_sweep();
                self.next_ms = now.saturating_add(REST_MS);
            }
            Tick::Stalled => {
                if self.sweep.stalled <= STALLS_SAID {
                    Line::new().text(b"scan gave no completion within its budget").send();
                }
                self.results.end_sweep();
                self.next_ms = now.saturating_add(REST_MS);
            }
            Tick::Failed => self.firmware_failed(now, Failure::Lost),
            Tick::Idle | Tick::Running => {}
        }
    }
}

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

//! A follow started, and paced: when the process table is read, and for how long it has run.

use alloc::vec::Vec;

use super::{Follow, FIRST_PROBE_MS, LOOK_GAP_MS};

impl Follow {
    pub fn new(before: Vec<u32>, now_ms: i64) -> Self {
        Follow {
            before,
            started_ms: now_ms,
            next_probe_ms: now_ms.saturating_add(FIRST_PROBE_MS),
            probe_gap_ms: FIRST_PROBE_MS,
            next_look_ms: now_ms,
            served: false,
        }
    }

    /// Whether this tick should read the process table at all.
    pub fn due(&mut self, now_ms: i64) -> bool {
        if now_ms < self.next_look_ms {
            return false;
        }
        self.next_look_ms = now_ms.saturating_add(LOOK_GAP_MS);
        true
    }

    /// How long the load has run, which bounds the age of a child it made.
    pub fn elapsed_ms(&self, now_ms: i64) -> i64 {
        now_ms.saturating_sub(self.started_ms)
    }
}

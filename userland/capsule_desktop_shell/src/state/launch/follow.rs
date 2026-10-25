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

//! A launch being followed: begun, and looked for no more often than the
//! registry needs.

use alloc::vec::Vec;

use super::timing::{FIRST_PROBE_MS, LOOK_GAP_MS};
use crate::state::Uptime;

pub struct Launch {
    pub name: Vec<u8>,
    /// The shell's children before the load, so a new one can be told apart.
    pub children: Vec<u32>,
    pub(super) started_ms: i64,
    pub(super) next_probe_ms: i64,
    pub(super) probe_gap_ms: i64,
    pub(super) next_look_ms: i64,
    pub(super) served: bool,
}

impl Launch {
    pub fn new(name: &[u8], children: Vec<u32>, now: Uptime) -> Self {
        let now_ms = now.0;
        Launch {
            name: name.to_vec(),
            children,
            started_ms: now_ms,
            next_probe_ms: now_ms.saturating_add(FIRST_PROBE_MS),
            probe_gap_ms: FIRST_PROBE_MS,
            next_look_ms: now_ms,
            served: false,
        }
    }

    /// Whether this turn should look for the app at all: the shell turns once
    /// per message it serves, and the registry need not be asked that often.
    pub fn due(&mut self, now: Uptime) -> bool {
        let now_ms = now.0;
        if now_ms < self.next_look_ms {
            return false;
        }
        self.next_look_ms = now_ms.saturating_add(LOOK_GAP_MS);
        true
    }
}

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

//! Closing the handles of callers that ended without closing them
//! (`handles.rs`): looked for at most every REAP_GAP_MS while requests
//! arrive, and before any request when the table is full.

use nonos_libc::{mk_pid_alive, mk_uptime_ms};

use crate::handles::HandleTable;

const REAP_GAP_MS: i64 = 2_000;

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

/// When the handles of ended callers were last looked for.
pub(super) struct Reaper {
    last: Option<i64>,
}

impl Reaper {
    pub(super) const fn new() -> Self {
        Self { last: None }
    }

    pub(super) fn reap_if_due(&mut self, handles: &mut HandleTable) {
        let now = mk_uptime_ms();
        let due = match self.last {
            Some(last) => now.wrapping_sub(last) >= REAP_GAP_MS,
            None => true,
        };
        if !due && !handles.is_full() {
            return;
        }
        self.last = Some(now);
        let _ = handles.close_ended(alive);
    }
}

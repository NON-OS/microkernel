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

//! Ending the streams of callers that ended without closing them
//! (`stream::orphaned`), looked for every REAP_GAP_S seconds.

use nonos_libc::mk_pid_alive;

use crate::manager::Manager;
use crate::stream::orphaned;

use super::handlers::end_stream;

const REAP_GAP_S: u64 = 2;

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

/// True when it looked, so the SOCKS front looks for its ended callers in
/// the same beat.
pub fn reap_due(state: &mut Manager, now: u64, last: &mut u64) -> bool {
    if *last != 0 && now >= *last && now - *last < REAP_GAP_S {
        return false;
    }
    *last = now;
    for id in orphaned(&state.streams, alive) {
        if let Some(index) = state.streams.iter().position(|s| s.id == id) {
            end_stream(state, index);
        }
    }
    true
}

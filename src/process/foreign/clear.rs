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

//! Taking a process out of the foreign table as it dies.

/// Drop `pid` from the table, whether it was a guest or a supervisor.
/// Called from process teardown; a supervisor leaving takes its guests.
pub fn clear(pid: u32) {
    let orphans = super::registry::guests_of(pid);
    /*
     * Each guest ends by the teardown a kill uses, so a thread running on
     * another CPU is sent off it and reaped only once no CPU is on it. Only
     * answering the parked ones left a running guest with no one to serve
     * its calls, holding its memory and its CPU for good. Its row still names
     * this supervisor while it goes, so no call of its is refused in the
     * meantime; the row goes in its own teardown, once it is a zombie. This
     * supervisor is a zombie already, and a guest's teardown ends no one but
     * the guests it hosts itself, so nothing here comes back to it.
     */
    for &guest in &orphans {
        crate::process::exit::teardown(guest, GUEST_END_CODE, true);
    }
    // Both directions go, not just this process's own row.
    super::registry::drop_rows(pid);
    for guest in orphans {
        // A guest already gone from the process table skipped its teardown.
        super::trap_reply::forget(guest);
        super::trap_frame::drop_frame(guest);
    }
    /*
     * A guest that died while parked leaves its frame behind, and
     * `take_answer` finds a frame by pid alone, so a reused pid would collect
     * an answer meant for a process that no longer exists.
     */
    super::trap_reply::forget(pid);
    super::trap_frame::drop_frame(pid);
    super::notice::forget_supervisor(pid);
}

// What a guest ended with its supervisor reads as: killed, as by SIGKILL.
const GUEST_END_CODE: i32 = 128 + crate::process::signal::SIGKILL as i32;

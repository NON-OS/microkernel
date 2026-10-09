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

//! Recording a new guest, only under a supervisor that is still alive.

use crate::process::core::ProcessState;

/// Record `pid` as a guest of `supervisor`. False if it is recorded already,
/// or if the supervisor is dying; the caller then ends `pid` itself.
pub(super) fn enrol(pid: u32, supervisor: u32) -> bool {
    if !super::registry::insert(pid, supervisor) {
        return false;
    }
    /*
     * A supervisor killed from another CPU can still be inside a call that
     * makes a guest. Its teardown marks it a zombie first and then collects
     * its guests from the table; this records the row first and then looks
     * at the supervisor. So either the teardown finds this row and ends the
     * guest, or this finds a zombie and takes the row back. Neither lock is
     * held while the other is taken.
     */
    if alive(supervisor) {
        return true;
    }
    super::registry::remove(pid);
    false
}

fn alive(pid: u32) -> bool {
    crate::process::with_process(pid, |pcb| {
        !matches!(*pcb.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
    })
    .unwrap_or(false)
}

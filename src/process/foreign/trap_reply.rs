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

//! The supervisor's answer, and what happens to a guest left without one.

use super::registry;
use super::trap_table::PARKED;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};

/*
 * A guest whose supervisor died is not left asleep forever and is not
 * told its call succeeded. It gets a refusal it can act on.
 */
const ABANDONED: u64 = ERRNO_NOENT as u64;

/// `MkForeignReply`: answer one parked guest. Refused unless the caller is
/// that guest's recorded supervisor, so a pid alone buys nothing.
pub fn sys_foreign_reply(pid: u64, value: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let pid = pid as u32;
    if registry::supervisor_of(pid) != Some(caller) {
        return ERRNO_PERM;
    }
    let mut parked = PARKED.lock();
    let Some(entry) = parked.iter_mut().find(|p| p.frame.pid == pid && p.answer.is_none()) else {
        return ERRNO_NOENT;
    };
    entry.answer = Some(value);
    drop(parked);
    crate::sched::wake_process(pid);
    0
}

/// Release every frame belonging to a guest whose supervisor has gone.
pub(super) fn abandon(pid: u32) {
    let mut parked = PARKED.lock();
    for entry in parked.iter_mut().filter(|p| p.frame.pid == pid) {
        entry.answer = Some(ABANDONED);
    }
    drop(parked);
    crate::sched::wake_process(pid);
}

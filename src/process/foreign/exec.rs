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

//! `MkForeignExec`: the same guest, a different program.

use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};

use super::exec_context::fresh;
use super::exec_swap::{drop_tls, swap};
use super::peer_guard::{in_user_half, pid_arg, supervised_asid};

pub fn sys_foreign_exec(pid: u64, entry: u64, rsp: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let pid = match pid_arg(pid) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if super::registry::supervisor_of(pid) != Some(caller) {
        return ERRNO_PERM;
    }
    if rsp == 0 || !in_user_half(entry, 1) || !in_user_half(rsp, 1) {
        return ERRNO_INVAL;
    }
    /*
     * A thread stopped at a tick is in no call an exec could answer: the stop
     * takes only a handler, so the thread would run on in the old image over
     * a context and a thread pointer already replaced. Refused as for a
     * thread not parked at all.
     */
    if super::trap_table::parked_nr(pid) == Some(super::frame::NR_INTERRUPTED) {
        return ERRNO_NOENT;
    }
    /* Read while the stack is the supervisor's alone, applied once it runs. */
    let name = supervised_asid(caller, u64::from(pid))
        .ok()
        .and_then(|(asid, _held)| super::guest_name::from_stack(asid, rsp));
    let Some(previous) = swap(pid, Some(fresh(entry, rsp))) else {
        return ERRNO_INVAL;
    };
    drop_tls(pid);
    // Answering is what releases the guest.
    match super::trap_reply::answer_raw(pid, super::trap_table::Answer::Execed) {
        0 => {
            if let Some(name) = name {
                super::guest_stats::rename(pid, name);
            }
            0
        }
        err => {
            swap(pid, previous);
            err
        }
    }
}

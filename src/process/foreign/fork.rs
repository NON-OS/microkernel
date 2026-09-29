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

//! Duplicating a guest.

use super::peer_guard::pid_arg;
use crate::process::core::ProcessState;
use crate::syscall::microkernel::errnos::{ERRNO_INVAL, ERRNO_NOENT, ERRNO_PERM};

/// `MkForeignFork`: a second process holding the first one's register state,
/// with zero in its return register so the two can tell each other apart,
/// which is the whole of fork's contract to the program.
pub fn sys_foreign_fork(pid: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let parent = match pid_arg(pid) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if super::registry::supervisor_of(parent) != Some(caller) {
        return ERRNO_PERM;
    }
    let Some(state) = saved_state(parent) else {
        // A guest that is not parked inside a syscall has no frame to copy.
        return ERRNO_NOENT;
    };
    let comm = super::guest_stats::comm_of(parent);
    let child = match super::spawn::empty_guest(caller, comm.as_bytes()) {
        Ok(pid) => pid,
        Err(e) => return e,
    };
    let mut frame = state;
    frame.rax = 0;
    // The thread pointer is a register the frame does not carry, so the child
    // takes its forking thread's, read from that thread's PCB. Without this a
    // fork from a thread that set its own FS would give the child a zero one.
    let parent_tls = crate::process::with_process(parent, |pcb| pcb.get_tls_base()).unwrap_or(0);
    crate::process::with_process(child, |pcb| {
        *pcb.saved_user_context.lock() = Some(frame);
        if parent_tls != 0 {
            pcb.set_tls_base(parent_tls);
        }
        *pcb.state.lock() = ProcessState::New;
    });
    child as i64
}

fn saved_state(pid: u32) -> Option<crate::arch::context::SavedUser> {
    super::trap_frame::parked_frame(pid)
}

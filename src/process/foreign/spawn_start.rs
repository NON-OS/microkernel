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

//! Making a built guest runnable.

use crate::kernel_core::process_spawn::{allocate_user_stack, setup_initial_user_context};
use crate::process::core::ProcessState;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};

/*
 * `rsp` of zero asks for the kernel's own user stack. Any other value is
 * a stack the supervisor built inside the guest, which is what a program
 * that reads its arguments needs: the kernel knows nothing about argument
 * vectors, and the supervisor that does cannot install a stack pointer
 * without this call.
 */
pub fn sys_foreign_start(pid: u64, entry: u64, rsp: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    let pid = pid as u32;
    if super::registry::supervisor_of(pid) != Some(caller) {
        return ERRNO_PERM;
    }
    let stack = match rsp {
        0 => match allocate_user_stack(pid) {
            Ok(top) => top,
            Err(_) => return ERRNO_NOMEM,
        },
        given => given,
    };
    if setup_initial_user_context(pid, entry, stack).is_err() {
        return ERRNO_FAULT;
    }
    crate::process::with_process(pid, |pcb| {
        *pcb.state.lock() = ProcessState::Ready;
    });
    crate::sched::add_to_run_queue(pid);
    0
}

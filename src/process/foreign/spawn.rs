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

//! Creating a guest: a process, a kernel stack, and nothing else.
//!
//! The kernel parses no image here. It hands back an empty process with no
//! capabilities, and the supervisor fills the address space itself through
//! the peer calls. Two steps rather than one, so a half-built guest never
//! becomes runnable: this file makes it, `spawn_start` runs it.

use alloc::format;

use crate::kernel_core::process_spawn::allocate_kernel_stack;
use crate::process::core::types::Priority;
use crate::process::core::{create_process_with_parent, ProcessState};
use crate::syscall::microkernel::errnos::{ERRNO_EXIST, ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM};
use crate::usercopy::read_user_bytes;

const MAX_NAME: usize = 24;

/// `MkForeignSpawn`: an empty, capability-free process supervised by the
/// caller. Returns its pid. The name is for the process table and the logs
/// only; it grants nothing and is not a service name.
pub fn sys_foreign_spawn(name_ptr: u64, name_len: u64) -> i64 {
    let Some(caller) = crate::process::current_pid() else {
        return ERRNO_INVAL;
    };
    if name_len == 0 || name_len as usize > MAX_NAME {
        return ERRNO_INVAL;
    }
    let Ok(bytes) = read_user_bytes(name_ptr, name_len as usize) else {
        return ERRNO_FAULT;
    };
    let Ok(name) = core::str::from_utf8(&bytes) else {
        return ERRNO_INVAL;
    };
    let tag = format!("foreign:{name}");
    let Ok(pid) = create_process_with_parent(&tag, ProcessState::New, Priority::Normal, 0, None)
    else {
        return ERRNO_NOMEM;
    };
    if allocate_kernel_stack(pid).is_err() {
        return ERRNO_NOMEM;
    }
    /*
     * No capabilities are installed. The contract gate then refuses every
     * NONOS syscall this process can name, which is the confinement
     * itself: the supervisor's own policy is a second layer, not the
     * only one.
     */
    if !super::registry::insert(pid, caller) {
        return ERRNO_EXIST;
    }
    pid as i64
}

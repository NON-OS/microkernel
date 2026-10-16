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

use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};

pub fn sys_thread_spawn(entry: u64, stack: u64) -> i64 {
    // Refused before anything is built, on the user entry builder's terms.
    if !crate::process::core::start_in_user_half(entry, stack) {
        return ERRNO_INVAL;
    }
    match crate::process::core::spawn_thread(entry, stack) {
        Ok(tid) => tid as i64,
        Err(_) => ERRNO_NOMEM,
    }
}

pub fn sys_set_tls(base: u64) -> i64 {
    // Zero clears the base; anything else must point at readable user
    // memory (userspace keeps the TLS table's self-pointer at offset 0,
    // so the first eight bytes are the smallest meaningful window).
    if base != 0 && crate::usercopy::validate_user_read(base, 8).is_err() {
        return ERRNO_FAULT;
    }
    let Some(pid) = current_pid() else {
        return ERRNO_PERM;
    };
    let Some(pcb) = crate::process::get_process_table().find_by_pid(pid) else {
        return ERRNO_INVAL;
    };
    if !crate::arch::context::set_user_tls(base) {
        // The arch cannot carry a user TLS register across its return
        // path yet; refuse rather than let userspace believe TLS works.
        return ERRNO_INVAL;
    }
    pcb.set_tls_base(base);
    0
}

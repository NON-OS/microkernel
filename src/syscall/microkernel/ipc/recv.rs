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

extern crate alloc;

use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};

use super::inbox_name::resolve_for_recv;
use super::recv_inbox::recv_from_inbox;

pub fn sys_ipc_recv(endpoint: u64, buf: u64, len: usize, timeout_ms: u64) -> i64 {
    if len == 0 {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_write(buf, len).is_err() {
        return ERRNO_FAULT;
    }
    let pid = current_pid().unwrap_or(0);
    let inbox_name = match resolve_for_recv(endpoint, pid) {
        Ok(name) => name,
        Err(e) => return e,
    };
    recv_from_inbox(pid, &inbox_name, buf, len, timeout_ms)
}

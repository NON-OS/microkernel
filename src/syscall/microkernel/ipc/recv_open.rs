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

//! What MkIpcRecvFrom and MkIpcRecvReq check before they wait: a buffer
//! that takes bytes, and an inbox the caller may receive on.

extern crate alloc;

use alloc::string::String;

use crate::ipc::nonos_inbox;
use crate::process::current_pid;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOENT};

use super::inbox_name::resolve_for_recv;

/// A receive buffer: EINVAL when empty, EFAULT when not writable.
pub(super) fn buffer(buf: u64, len: usize) -> Result<(), i64> {
    if len == 0 {
        return Err(ERRNO_INVAL);
    }
    crate::usercopy::validate_user_write(buf, len).map_err(|_| ERRNO_FAULT)
}

/// The inbox `endpoint` names for the caller: its resolution's error, or
/// ENOENT when it does not exist.
pub(super) fn inbox(endpoint: u64) -> Result<String, i64> {
    let name = resolve_for_recv(endpoint, current_pid().unwrap_or(0))?;
    if !nonos_inbox::exists(&name) {
        return Err(ERRNO_NOENT);
    }
    Ok(name)
}

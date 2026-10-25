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

//! `MkForeignSpawn`: a supervisor asks for a guest by name.

pub(super) use super::spawn_guest::empty_guest;
use crate::syscall::microkernel::errnos::{ERRNO_FAULT, ERRNO_INVAL};
use crate::usercopy::read_user_bytes;

const MAX_NAME: usize = 24;

/// `MkForeignSpawn`: an empty, capability-free process supervised by the
/// caller.
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
    match empty_guest(caller, name.as_bytes()) {
        Ok(pid) => pid as i64,
        Err(e) => e,
    }
}

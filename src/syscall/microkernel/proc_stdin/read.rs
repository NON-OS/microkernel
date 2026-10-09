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

//! `MkStdinRead`: a child drains its own `stdin.<pid>` inbox.
//!
//! The inbox is a byte stream in the order it was fed: a read with a buffer
//! shorter than the next message takes what fits and leaves the rest first
//! in line, so nothing is dropped. The buffer is checked writable before
//! anything is taken; only one unmapped between that check and the copy
//! loses what was taken, and the read then says EFAULT. The kernel's copy
//! is zeroed once handed on, since it can be what a person typed.

use super::super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};
use crate::process::current_pid;

pub fn sys_stdin_read(buf_ptr: u64, buf_len: usize) -> i64 {
    if buf_ptr == 0 || buf_len == 0 {
        return ERRNO_INVAL;
    }
    let caller = current_pid().unwrap_or(0);
    if caller == 0 {
        return ERRNO_PERM;
    }
    if crate::usercopy::validate_user_write(buf_ptr, buf_len).is_err() {
        return ERRNO_FAULT;
    }
    let name = alloc::format!("stdin.{}", caller);
    let Some(mut bytes) = crate::ipc::nonos_inbox::take_front(&name, buf_len) else {
        return 0;
    };
    let copied = crate::usercopy::copy_to_user(buf_ptr, &bytes);
    let n = bytes.len() as i64;
    crate::crypto::secure_zero(&mut bytes);
    match copied {
        Ok(_) => n,
        Err(_) => ERRNO_FAULT,
    }
}

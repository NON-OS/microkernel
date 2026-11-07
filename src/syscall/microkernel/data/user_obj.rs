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

//! The person's files' objects, for the user-files broker alone: each call
//! needs UserFiles, and names an object by number, never by path
//! (fs/blockfs_volume/user_obj).
//!
//! `MkUserObjPut(id, buf, len)`: object `id` written as `len` bytes, at most
//!   1 MiB, made or replaced. 0.
//! `MkUserObjRead(id, offset, buf, len)`: up to `len` bytes, at most 1 MiB,
//!   from `offset`. The bytes read, 0 at its end.
//! `MkUserObjSize(id)`: its length. ENOENT when there is none.
//! `MkUserObjDrop(id)`: removed. ENOENT when there was none.

use alloc::vec::Vec;

use super::super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};
use super::errno::errno;
use crate::fs::blockfs_volume::{
    open_machine_volume, user_put, user_read, user_remove, user_size, OBJECT_MAX,
};

fn allowed() -> Result<(), i64> {
    if !crate::syscall::caps::current_caps_or_default().can_user_files() {
        return Err(ERRNO_PERM);
    }
    open_machine_volume().map_err(errno)
}

/// A kernel buffer of `len` bytes, or ENOMEM; the heap wipes it as it frees it.
fn buffer(len: u64) -> Result<Vec<u8>, i64> {
    let mut out = Vec::new();
    out.try_reserve_exact(len as usize).map_err(|_| ERRNO_NOMEM)?;
    out.resize(len as usize, 0u8);
    Ok(out)
}

pub fn sys_user_obj_put(id: u64, buf: u64, len: u64) -> i64 {
    if let Err(e) = allowed() {
        return e;
    }
    if len > OBJECT_MAX as u64 || (len > 0 && buf == 0) {
        return ERRNO_INVAL;
    }
    let mut bytes = match buffer(len) {
        Ok(b) => b,
        Err(e) => return e,
    };
    if len > 0 && crate::usercopy::copy_from_user(buf, &mut bytes).is_err() {
        return ERRNO_FAULT;
    }
    user_put(id, &bytes).map_or_else(errno, |()| 0)
}

pub fn sys_user_obj_read(id: u64, offset: u64, buf: u64, len: u64) -> i64 {
    if let Err(e) = allowed() {
        return e;
    }
    if buf == 0 || len == 0 || len > OBJECT_MAX as u64 {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_write(buf, len as usize).is_err() {
        return ERRNO_FAULT;
    }
    let mut bytes = match buffer(len) {
        Ok(b) => b,
        Err(e) => return e,
    };
    let n = match user_read(id, offset, &mut bytes) {
        Ok(n) => n,
        Err(e) => return errno(e),
    };
    if crate::usercopy::copy_to_user(buf, &bytes[..n]).is_err() {
        return ERRNO_FAULT;
    }
    n as i64
}

pub fn sys_user_obj_size(id: u64) -> i64 {
    if let Err(e) = allowed() {
        return e;
    }
    user_size(id).map_or_else(errno, |n| n as i64)
}

pub fn sys_user_obj_drop(id: u64) -> i64 {
    if let Err(e) = allowed() {
        return e;
    }
    user_remove(id).map_or_else(errno, |()| 0)
}

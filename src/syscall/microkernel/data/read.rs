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

//! `MkDataRead(name, len, offset, buf, buf_len)`: read a range of a file on
//! the data volume into the caller's buffer, at most 4 MiB a call. Returns
//! the bytes read, 0 at the file's end.
//!
//! The bytes pass through a bounce buffer on the kernel heap, taken without
//! panicking: a heap that cannot spare it refuses the call with ENOMEM.

use alloc::vec::Vec;

use super::super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_NOMEM, ERRNO_PERM};
use super::errno::errno;
use super::name::copy_name;

const MAX_READ: u64 = 4 << 20;

pub fn sys_data_read(name_ptr: u64, name_len: u64, offset: u64, buf: u64, buf_len: u64) -> i64 {
    if !crate::syscall::caps::current_caps_or_default().can_open_files() {
        return ERRNO_PERM;
    }
    let (name, len) = match copy_name(name_ptr, name_len) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if buf == 0 || buf_len == 0 || buf_len > MAX_READ {
        return ERRNO_INVAL;
    }
    if crate::usercopy::validate_user_write(buf, buf_len as usize).is_err() {
        return ERRNO_FAULT;
    }
    if let Err(e) = crate::fs::blockfs_volume::open_machine_volume() {
        return errno(e);
    }
    let mut bounce = Vec::new();
    if bounce.try_reserve_exact(buf_len as usize).is_err() {
        crate::log::warn!("[DATA] read refused: no {} bytes of heap to bounce it", buf_len);
        return ERRNO_NOMEM;
    }
    bounce.resize(buf_len as usize, 0u8);
    let n = match crate::fs::blockfs_volume::read_at(&name[..len], offset, &mut bounce) {
        Ok(n) => n,
        Err(e) => return errno(e),
    };
    if crate::usercopy::copy_to_user(buf, &bounce[..n]).is_err() {
        return ERRNO_FAULT;
    }
    n as i64
}

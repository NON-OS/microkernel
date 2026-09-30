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

//! `MkDataRead(name, len, offset, buf, buf_len, peer)`: read a range of a
//! file on the data volume, at most 4 MiB a call. Returns the bytes read, 0
//! at the file's end.
//!
//! With `peer` 0 the bytes go to the caller's buffer (`read_bounce`);
//! otherwise `buf` is an address in the guest `peer`, which the caller
//! supervises (`read_peer`).

use super::super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};
use super::errno::errno;
use super::name::copy_name;

const MAX_READ: u64 = 4 << 20;

pub fn sys_data_read(
    name_ptr: u64,
    name_len: u64,
    offset: u64,
    buf: u64,
    buf_len: u64,
    peer: u64,
) -> i64 {
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
    if peer == 0 && crate::usercopy::validate_user_write(buf, buf_len as usize).is_err() {
        return ERRNO_FAULT;
    }
    if let Err(e) = crate::fs::blockfs_volume::open_machine_volume() {
        return errno(e);
    }
    if peer != 0 {
        return super::read_peer::read_into_guest(&name[..len], offset, peer, buf, buf_len);
    }
    super::read_bounce::read_to_caller(&name[..len], offset, buf, buf_len)
}

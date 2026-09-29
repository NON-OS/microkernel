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

//! `MkDataImport(name, len, sha256)`: bring the disk plan's file onto the
//! volume as `name`, kept only if its SHA-256 is the 32 bytes the caller
//! pins. Returns the file's size.

use super::super::errnos::{ERRNO_FAULT, ERRNO_PERM};
use super::errno::errno;
use super::name::copy_name;

pub fn sys_data_import(name_ptr: u64, name_len: u64, digest_ptr: u64) -> i64 {
    let caps = crate::syscall::caps::current_caps_or_default();
    if !(caps.can_store_write() && caps.can_open_files()) {
        return ERRNO_PERM;
    }
    let (name, len) = match copy_name(name_ptr, name_len) {
        Ok(n) => n,
        Err(e) => return e,
    };
    let mut want = [0u8; 32];
    if digest_ptr == 0 || crate::usercopy::copy_from_user(digest_ptr, &mut want).is_err() {
        return ERRNO_FAULT;
    }
    match crate::fs::blockfs_volume::import(&name[..len], &want) {
        Ok(done) => done.bytes as i64,
        Err(e) => errno(e),
    }
}

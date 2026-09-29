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

//! `MkDataStat(name, len)`: the size of a file on the data volume.

use super::super::errnos::{ERRNO_INVAL, ERRNO_PERM};
use super::errno::errno;
use super::name::copy_name;

pub fn sys_data_stat(name_ptr: u64, name_len: u64) -> i64 {
    if !crate::syscall::caps::current_caps_or_default().can_open_files() {
        return ERRNO_PERM;
    }
    let (name, len) = match copy_name(name_ptr, name_len) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if let Err(e) = crate::fs::blockfs_volume::open_machine_volume() {
        return errno(e);
    }
    match crate::fs::blockfs_volume::stat(&name[..len]) {
        Ok((_, true)) => ERRNO_INVAL,
        Ok((size, false)) => size as i64,
        Err(e) => errno(e),
    }
}

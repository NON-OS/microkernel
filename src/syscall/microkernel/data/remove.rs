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


/*
 * `MkDataRemove(name_ptr, name_len)`: take an imported file off the data
 * volume, with its record. StreamImport, the right that brings one in, is
 * the right that takes one out. A download put down part way goes too.
 * 0; ENOENT when nothing is there, EBUSY while a stream is feeding the
 * name, EPERM for a record or a mark.
 */

use super::super::errnos::ERRNO_PERM;
use super::errno::errno;
use super::name::copy_name;

pub fn sys_data_remove(name_ptr: u64, name_len: u64) -> i64 {
    if !crate::syscall::caps::current_caps_or_default().can_stream_import() {
        return ERRNO_PERM;
    }
    let (name, len) = match copy_name(name_ptr, name_len) {
        Ok(n) => n,
        Err(e) => return e,
    };
    if let Err(e) = crate::fs::blockfs_volume::open_machine_volume() {
        return errno(e);
    }
    match crate::fs::blockfs_volume::remove_import(&name[..len]) {
        Ok(()) => 0,
        Err(crate::fs::blockfs_volume::VolumeError::ImportOnly) => ERRNO_PERM,
        Err(e) => errno(e),
    }
}

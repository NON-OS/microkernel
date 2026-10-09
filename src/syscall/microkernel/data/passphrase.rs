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

//! `MkDataPassphrase(create, passphrase, len)`: key the data volume with a
//! passphrase. With `create` 1 a new volume is made over a header ring that
//! has never been written; with 0 the volume the key header says a
//! passphrase keys is opened. The passphrase is copied into kernel memory,
//! used, and wiped; it is never logged. Needs StoreWrite and FileSystem.
//! Returns 0; EACCES for a wrong passphrase, ENOENT for a volume no
//! passphrase keys, EEXIST for a create over a volume, EBUSY when a volume
//! is open already, ENOMEM when Argon2id finds no memory, EAGAIN while no
//! disk is chosen yet.

use super::super::errnos::{ERRNO_FAULT, ERRNO_INVAL, ERRNO_PERM};
use super::errno::errno;
use crate::crypto::constant_time::secure_zero;

/// The longest passphrase taken, and the shortest a new volume is keyed by.
const PASSPHRASE_MAX: usize = 256;
const CREATE_MIN: usize = 8;

pub fn sys_data_passphrase(create: u64, ptr: u64, len: u64) -> i64 {
    let caps = crate::syscall::caps::current_caps_or_default();
    if !(caps.can_store_write() && caps.can_open_files()) {
        return ERRNO_PERM;
    }
    let min = match create {
        0 => 1,
        1 => CREATE_MIN,
        _ => return ERRNO_INVAL,
    };
    let len = len as usize;
    if ptr == 0 || !(min..=PASSPHRASE_MAX).contains(&len) {
        return ERRNO_INVAL;
    }
    let mut buf = [0u8; PASSPHRASE_MAX];
    let rc = if crate::usercopy::copy_from_user(ptr, &mut buf[..len]).is_err() {
        ERRNO_FAULT
    } else {
        match crate::fs::blockfs_volume::passphrase_volume(create == 1, &buf[..len]) {
            Ok(()) => 0,
            Err(e) => errno(e),
        }
    };
    secure_zero(&mut buf);
    rc
}

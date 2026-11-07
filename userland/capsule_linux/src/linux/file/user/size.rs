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

//! Setting a broker file's size, and syncing it.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::{link, owns_fd, place};

/// ftruncate on a broker file; None when `fd` is not one. The descriptor
/// learns the length only once the broker has it.
pub fn ftruncate(fd: &mut Fd, len: u64) -> Option<u64> {
    let handle = owns_fd(fd)?;
    if !fd.writable {
        return Some(errno::fail(errno::EINVAL));
    }
    Some(match link::with(|c| c.ftruncate(handle, len)) {
        Ok(()) => {
            fd.size = len;
            errno::ok(0)
        }
        Err(e) => errno::fail(e as i64),
    })
}

/// truncate by name, for a guest served the folders.
pub fn truncate(guest: &Guest, full: &[u8], len: u64) -> Option<u64> {
    let path = place::served(guest, full)?;
    Some(
        link::with(|c| c.truncate(path, len))
            .map_or_else(|e| errno::fail(e as i64), |()| errno::ok(0)),
    )
}

/// fsync and fdatasync: the broker commits every changed chunk and the
/// tree before it answers.
pub fn fsync(fd: &Fd) -> Option<u64> {
    let handle = owns_fd(fd)?;
    Some(link::with(|c| c.fsync(handle)).map_or_else(|e| errno::fail(e as i64), |()| errno::ok(0)))
}

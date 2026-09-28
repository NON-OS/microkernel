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
 * `write` on a file: at the descriptor's offset, or at the end when it
 * was opened O_APPEND, and the offset moves to where the write ended.
 */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::rw::{write_at, MAX_IO};

pub fn write(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let take = (len as usize).min(MAX_IO);
    let Some(bytes) = guest.read(buf, take) else {
        return errno::fail(errno::EFAULT);
    };
    let at = guest.fds.get(fd as usize).map_or(0, super::desc::pos);
    match write_at(guest, fd, at, &bytes) {
        Ok((n, end)) => {
            if let Some(entry) = guest.fds.get_mut(fd as usize) {
                super::desc::set_pos(entry, end);
            }
            errno::ok(n as u64)
        }
        Err(e) => errno::fail(e),
    }
}

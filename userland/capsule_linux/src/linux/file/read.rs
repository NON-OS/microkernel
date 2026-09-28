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

/* `read` on a file: the bytes at the descriptor's offset, which moves on. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::rw::read_at;

pub fn read(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let at = guest.fds.get(fd as usize).map_or(0, super::desc::pos);
    let bytes = match read_at(guest, fd, at, len as usize) {
        Ok(bytes) => bytes,
        Err(e) => return errno::fail(e),
    };
    if bytes.is_empty() {
        return errno::ok(0);
    }
    if guest.write(buf, &bytes) < bytes.len() as i64 {
        return errno::fail(errno::EFAULT);
    }
    if let Some(entry) = guest.fds.get_mut(fd as usize) {
        super::desc::set_pos(entry, at + bytes.len() as u64);
    }
    errno::ok(bytes.len() as u64)
}

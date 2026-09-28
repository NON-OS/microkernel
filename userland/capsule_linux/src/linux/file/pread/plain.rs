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

/* pread64 and pwrite64, and the offset they use and give back. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::rw::{write_at, MAX_IO};

pub fn pread64(guest: &mut Guest, fd: u64, buf: u64, len: u64, at: u64) -> u64 {
    at_offset(guest, fd, at, |g| super::super::read::read(g, fd, buf, len))
}

pub fn pwrite64(guest: &mut Guest, fd: u64, buf: u64, len: u64, at: u64) -> u64 {
    if (at as i64) < 0 {
        return errno::fail(errno::EINVAL);
    }
    if let Err(e) = seekable(guest, fd) {
        return e;
    }
    let Some(bytes) = guest.read(buf, (len as usize).min(MAX_IO)) else {
        return errno::fail(errno::EFAULT);
    };
    match write_at(guest, fd, at, &bytes) {
        Ok((n, _)) => errno::ok(n as u64),
        Err(e) => errno::fail(e),
    }
}

pub(super) fn seekable(guest: &Guest, fd: u64) -> Result<u64, u64> {
    match guest.fds.get(fd as usize).filter(|f| f.is_open()) {
        Some(f) if f.kind == Kind::File => Ok(super::super::desc::pos(f)),
        Some(f) if f.kind == Kind::Dir => Err(errno::fail(errno::EISDIR)),
        Some(_) => Err(errno::fail(errno::ESPIPE)),
        None => Err(errno::fail(errno::EBADF)),
    }
}

/* Run `go` with the descriptor's offset set to `at`, then put it back. */
pub(super) fn at_offset(
    guest: &mut Guest,
    fd: u64,
    at: u64,
    go: impl FnOnce(&mut Guest) -> u64,
) -> u64 {
    if (at as i64) < 0 {
        return errno::fail(errno::EINVAL);
    }
    let saved = match seekable(guest, fd) {
        Ok(saved) => saved,
        Err(e) => return e,
    };
    super::super::desc::set_pos(&mut guest.fds[fd as usize], at);
    let out = go(guest);
    if let Some(entry) = guest.fds.get_mut(fd as usize) {
        super::super::desc::set_pos(entry, saved);
    }
    out
}

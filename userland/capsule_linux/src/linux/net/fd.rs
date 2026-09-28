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

//! From a descriptor to the family socket it names, and back.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

use super::sock;

/// SOCK_NONBLOCK and SOCK_CLOEXEC, which socket, socketpair and accept4
/// take with the type or as flags: O_NONBLOCK and O_CLOEXEC's values.
pub const SOCK_NONBLOCK: u64 = 0o4000;
pub const SOCK_CLOEXEC: u64 = 0o2_000_000;

/// The socket `fd` names, or the errno Linux gives for a descriptor that is
/// not one: EBADF when nothing is open there, ENOTSOCK when a file is.
pub fn sock_of(guest: &Guest, fd: u64) -> Result<u32, u64> {
    match guest.fds.get(fd as usize) {
        Some(f) if f.kind == Kind::Socket => Ok(f.handle),
        Some(f) if f.is_open() => Err(errno::fail(errno::ENOTSOCK)),
        _ => Err(errno::fail(errno::EBADF)),
    }
}

/// A descriptor for socket `id`, with the flags asked for. A socket with no
/// descriptor to name it is let go at once, since nobody could close it.
pub fn install(guest: &mut Guest, id: u32, flags: u64) -> u64 {
    match crate::linux::file::install(guest, Fd::socket(id)) {
        Some(n) => {
            if let Some(f) = guest.fds.get_mut(n as usize) {
                f.nonblock = flags & SOCK_NONBLOCK != 0;
                f.cloexec = flags & SOCK_CLOEXEC != 0;
            }
            errno::ok(n)
        }
        None => {
            sock::with(|t| t.release(id, guest.pid));
            errno::fail(errno::EMFILE)
        }
    }
}

/// True when `fd` is non-blocking.
pub fn nonblock(guest: &Guest, fd: u64) -> bool {
    guest.fds.get(fd as usize).is_some_and(|f| f.nonblock)
}

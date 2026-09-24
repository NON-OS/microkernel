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

//! `read`, `write` and `close`. A guest's descriptor is an index into a
//! table this capsule owns, so the number it passes can only ever reach
//! something this capsule decided to give it.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

/// Cap on one transfer, matching the kernel's own peer-copy ceiling.
const MAX_IO: u64 = 1 << 20;

pub fn write(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let Some(entry) = guest.fds.get(fd as usize).copied() else {
        return errno::fail(errno::EBADF);
    };
    match entry.kind {
        Kind::Stdout | Kind::Stderr => console(guest, buf, len),
        Kind::Stdin => errno::fail(errno::EBADF),
        Kind::Free => errno::fail(errno::EBADF),
    }
}

/// A guest's console output, carried to the host's log. The bytes are the
/// guest's and are never interpreted, only forwarded.
fn console(guest: &Guest, buf: u64, len: u64) -> u64 {
    if len == 0 {
        return errno::ok(0);
    }
    let take = len.min(MAX_IO);
    let Some(bytes) = guest.read(buf, take as usize) else {
        return errno::fail(errno::EFAULT);
    };
    let _ = nonos_libc::mk_debug(bytes.as_ptr(), bytes.len());
    errno::ok(take)
}

pub fn read(guest: &mut Guest, fd: u64, _buf: u64, _len: u64) -> u64 {
    match guest.fds.get(fd as usize).map(|f| f.kind) {
        /* Nothing is typed at a guest yet, and end of file is the truth. */
        Some(Kind::Stdin) => errno::ok(0),
        Some(Kind::Stdout) | Some(Kind::Stderr) => errno::fail(errno::EBADF),
        Some(Kind::Free) | None => errno::fail(errno::EBADF),
    }
}

pub fn close(guest: &mut Guest, fd: u64) -> u64 {
    match guest.fds.get_mut(fd as usize) {
        Some(entry) if entry.is_open() => {
            entry.kind = Kind::Free;
            errno::ok(0)
        }
        _ => errno::fail(errno::EBADF),
    }
}

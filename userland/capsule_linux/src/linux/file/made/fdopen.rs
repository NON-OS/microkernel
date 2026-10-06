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
 * Opening /proc/<pid>/fd/<n> when it names no path: a pipe, the console, a
 * socket, or an object with no file behind it.
 *
 * On Linux such an open reaches the same pipe again, and fails with ENXIO
 * for a socket or an anonymous object. For the asking process's own pipe
 * that is a second descriptor on the same pipe, which is what dup makes.
 * Another process's descriptors are not reached this way here: that would
 * hand one process a way into what another holds.
 */

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::super::flags::O_CLOEXEC;
use super::super::slot;
use super::proc::number;
use super::view;

pub fn reopen(guest: &mut Guest, path: &[u8], to: &[u8], flags: u64) -> u64 {
    if !to.starts_with(b"pipe:") {
        return errno::fail(errno::ENXIO);
    }
    let parts: alloc::vec::Vec<&[u8]> =
        path.split(|b| *b == b'/').filter(|p| !p.is_empty()).collect();
    let (Some(pid), Some(n)) =
        (parts.get(1).and_then(|p| number(p)), parts.last().and_then(|p| number(p)))
    else {
        return errno::fail(errno::ENOENT);
    };
    if pid != view::with(|v| v.me) {
        let line = b"[LINUX] refused /proc/<pid>/fd of another process: a way into what it holds\n";
        let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
        return errno::fail(errno::EACCES);
    }
    let Some(from) = guest.fds.get(n as usize).filter(|f| f.is_open()) else {
        return errno::fail(errno::ENOENT);
    };
    let mut fd = Fd::clone_of(from);
    fd.cloexec = flags & O_CLOEXEC != 0;
    match slot::install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

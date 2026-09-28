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

//! `fcntl`.

use crate::linux::abi::errno;
use crate::linux::file;
use crate::linux::file::flags::{O_NONBLOCK, O_RDWR, O_WRONLY};
use crate::linux::guest::{Fd, Guest, Kind};

const F_DUPFD: u64 = 0;
const F_GETFD: u64 = 1;
const F_SETFD: u64 = 2;
const F_GETFL: u64 = 3;
const F_SETFL: u64 = 4;
const F_DUPFD_CLOEXEC: u64 = 1030;

/// The only descriptor flag there is.
const FD_CLOEXEC: u64 = 1;

pub fn fcntl(guest: &mut Guest, fd: u64, cmd: u64, arg: u64) -> u64 {
    let Some(entry) = guest.fds.get_mut(fd as usize).filter(|e| e.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    match cmd {
        /*
         * A shell sets close-on-exec on the descriptors it keeps for itself,
         * then execs, and expects the command not to see them.
         */
        F_GETFD => errno::ok(u64::from(entry.cloexec)),
        F_SETFD => {
            entry.cloexec = arg & FD_CLOEXEC != 0;
            errno::ok(0)
        }
        /*
         * O_NONBLOCK is the status flag that changes what a call does here,
         * so it is the one kept. The rest a program can set (O_APPEND,
         * O_ASYNC, O_DIRECT, O_NOATIME) are accepted and have no effect.
         */
        F_SETFL => {
            entry.nonblock = arg & O_NONBLOCK != 0;
            errno::ok(0)
        }
        F_GETFL => errno::ok(status(entry)),
        /* The lowest free number at or above `arg`: where a shell keeps one aside. */
        F_DUPFD | F_DUPFD_CLOEXEC => file::dup_from(guest, fd, arg, cmd == F_DUPFD_CLOEXEC),
        _ => errno::fail(errno::EINVAL),
    }
}

/// The access mode and O_NONBLOCK. A pipe's ends are read-only and
/// write-only, as `pipe2` makes them; anything else reads as read-write.
fn status(entry: &Fd) -> u64 {
    let mode = match entry.kind {
        Kind::Pipe if entry.writable => O_WRONLY,
        Kind::Pipe => 0,
        _ => O_RDWR,
    };
    mode | if entry.nonblock { O_NONBLOCK } else { 0 }
}

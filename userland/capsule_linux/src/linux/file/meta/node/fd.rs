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

/* The metadata for a descriptor, and the kind of file it is on. */

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

use super::super::super::modes;
use super::super::statbuf::Meta;
use super::device::device;
use super::path::{at, of, S_IFIFO, S_IFREG, S_IFSOCK};

/*
 * What has no path, a pipe, the console, a socket or an object with no
 * file behind it, by its mode alone.
 */
pub(super) fn kind(mode: u32) -> Meta {
    at(b"/", mode, 0, now())
}

/* fstat: a file by its path, and the rest by kind. */
pub fn of_fd(guest: &Guest, f: &Fd) -> Result<Meta, i64> {
    match f.kind {
        Kind::Free => Err(errno::EBADF),
        Kind::File | Kind::Dir => {
            let m = of(guest, f.path.clone(), true);
            /* A file this family is making exists before the store holds it. */
            m.or_else(|_| Ok(at(&f.path, S_IFREG | modes::FILE, f.size, now())))
        }
        /* The console is a stream with no terminal behind it, as a pipe is. */
        Kind::Stdin | Kind::Stdout | Kind::Stderr | Kind::Pipe => Ok(kind(S_IFIFO | 0o600)),
        Kind::Socket | Kind::Unix | Kind::Resolver => Ok(kind(S_IFSOCK | 0o777)),
        Kind::Memfd => Ok(Meta { size: f.size, ..at(b"/memfd:", S_IFREG | 0o777, 0, now()) }),
        Kind::Device => device(&f.path, f.handle).ok_or(errno::EBADF),
        Kind::Epoll | Kind::Timer | Kind::Event | Kind::Signal => Ok(kind(0o600)),
    }
}

pub fn now() -> u64 {
    u64::try_from(nonos_libc::mk_time_millis()).unwrap_or(0)
}

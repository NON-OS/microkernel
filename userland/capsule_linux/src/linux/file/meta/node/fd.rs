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
use super::super::super::proc::{CONSOLE_IN, CONSOLE_OUT, PIPES, SOCKETS};
use super::super::super::synth::S_IFREG;
use super::super::statbuf::Meta;
use super::device::device;
use super::made::named;
use super::path::{at, of};

/* fstat: a file by its path, and the rest by kind, as /proc names them. */
pub fn of_fd(guest: &Guest, f: &Fd) -> Result<Meta, i64> {
    match f.kind {
        Kind::Free => Err(errno::EBADF),
        Kind::File | Kind::Dir => {
            let m = of(guest, f.path.clone(), true);
            /* A file this family is making exists before the store holds it. */
            m.or_else(|_| Ok(at(&f.path, S_IFREG | modes::FILE, f.size, now())))
        }
        Kind::Stdin => Ok(named(&alloc::format!("pipe:[{CONSOLE_IN}]").into_bytes(), b"/")),
        Kind::Stdout | Kind::Stderr => {
            Ok(named(&alloc::format!("pipe:[{CONSOLE_OUT}]").into_bytes(), b"/"))
        }
        Kind::Pipe => {
            Ok(named(&alloc::format!("pipe:[{}]", PIPES + u64::from(f.handle)).into_bytes(), b"/"))
        }
        Kind::Socket | Kind::Unix | Kind::Resolver => Ok(named(
            &alloc::format!("socket:[{}]", SOCKETS + u64::from(f.handle)).into_bytes(),
            b"/",
        )),
        Kind::Memfd => Ok(Meta { size: f.size, ..at(b"/memfd:", S_IFREG | 0o777, 0, now()) }),
        Kind::Device => device(&f.path, f.handle).ok_or(errno::EBADF),
        Kind::Epoll | Kind::Timer | Kind::Event | Kind::Signal => Ok(named(b"anon_inode:[]", b"/")),
    }
}

pub fn now() -> u64 {
    u64::try_from(nonos_libc::mk_time_millis()).unwrap_or(0)
}

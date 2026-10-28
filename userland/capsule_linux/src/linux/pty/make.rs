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

//! A new pair's master, and a descriptor on either side of a pair.

use crate::linux::abi::errno;
use crate::linux::file::flags::{O_CLOEXEC, O_NONBLOCK};
use crate::linux::file::install;
use crate::linux::guest::{Fd, Guest, Kind};

use super::cell::with;
use super::name::{handle, pts_path};

/// /dev/ptmx: a pair at the lowest free number and a descriptor on its
/// master; ENOSPC past the most a family has, as devpts answers.
pub(super) fn master(guest: &mut Guest, flags: u64) -> u64 {
    let Some(n) = with(|t| t.make()) else {
        return errno::fail(errno::ENOSPC);
    };
    let Some(fd) = end(guest, n, true, flags) else {
        with(|t| t.pairs[n as usize] = None);
        return errno::fail(errno::EMFILE);
    };
    errno::ok(fd)
}

/// A descriptor on pair `n`'s master or slave, read and written.
pub(super) fn end(guest: &mut Guest, n: u32, master: bool, flags: u64) -> Option<u64> {
    let mut fd = Fd::empty(Kind::Pty);
    (fd.handle, fd.writable, fd.nonblock) = (handle(n, master), true, flags & O_NONBLOCK != 0);
    fd.cloexec = flags & O_CLOEXEC != 0;
    fd.path = if master { b"/dev/ptmx".to_vec() } else { pts_path(n) };
    install(guest, fd)
}

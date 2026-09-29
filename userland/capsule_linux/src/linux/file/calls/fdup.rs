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
 * fcntl's F_DUPFD and F_DUPFD_CLOEXEC: a second descriptor on the same open
 * file, at the lowest free number at or above the one asked for.
 *
 * The copy shares the description (held/desc/) and, for a file, the family's
 * copy of its bytes (held/cache/); a read through it opens its own stream
 * from the store when it needs one (held/rw/).
 */

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

use super::super::slot::MAX_FDS;

pub fn dup_from(guest: &mut Guest, fd: u64, min: u64, cloexec: bool) -> u64 {
    let Some(from) = guest.fds.get(fd as usize).filter(|f| f.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    /* RLIMIT_NOFILE is the table's size. */
    if min >= MAX_FDS as u64 {
        return errno::fail(errno::EINVAL);
    }
    let mut copy = Fd::clone_of(from);
    copy.cloexec = cloexec;
    let at = (min as usize..MAX_FDS).find(|i| !guest.fds.get(*i).is_some_and(|f| f.is_open()));
    let Some(at) = at else {
        return errno::fail(errno::EMFILE);
    };
    while guest.fds.len() <= at {
        guest.fds.push(Fd::empty(Kind::Free));
    }
    guest.fds[at] = copy;
    errno::ok(at as u64)
}

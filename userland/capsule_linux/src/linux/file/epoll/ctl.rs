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

//! epoll_ctl: one entry added, changed or dropped.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Watch};

use super::super::epoll_rules::{EPOLL_CTL_ADD, EPOLL_CTL_MOD};
use super::list::read_event;
use super::refuse::refused;

const EPOLL_CTL_DEL: u64 = 2;

/// Add, change or drop one entry, refused as Linux refuses it: a closed
/// descriptor is EBADF, a regular file or directory EPERM (always ready, so
/// never worth waiting on; Go's os.Open falls back to blocking reads on it),
/// watching the list itself EINVAL, EPOLLEXCLUSIVE where Linux does not
/// allow it EINVAL, an epoll that would end up watching itself or nest too
/// deep ELOOP, adding twice EEXIST, changing or dropping what is not there
/// ENOENT.
pub fn epoll_ctl(guest: &mut Guest, ep: u64, op: u64, fd: u64, event: u64) -> u64 {
    let entry = match op {
        EPOLL_CTL_DEL => None,
        EPOLL_CTL_ADD | EPOLL_CTL_MOD => match read_event(guest, event) {
            Some(pair) => Some(pair),
            None => return errno::fail(errno::EFAULT),
        },
        _ => return errno::fail(errno::EINVAL),
    };
    let present = match refused(guest, (ep, op, fd), entry.map(|(events, _)| events)) {
        Ok(present) => present,
        Err(e) => return e,
    };
    match op {
        EPOLL_CTL_ADD if present.is_some() => return errno::fail(errno::EEXIST),
        EPOLL_CTL_MOD | EPOLL_CTL_DEL if present.is_none() => return errno::fail(errno::ENOENT),
        _ => {}
    }
    let Some(list) = guest.fds.get_mut(ep as usize) else {
        return errno::fail(errno::EBADF);
    };
    // A change re-arms the entry: it is looked at afresh, as a new one is.
    list.watch.retain(|w| w.fd != fd);
    if let Some((events, data)) = entry {
        list.watch.push(Watch::new(fd, events, data));
    }
    errno::ok(0)
}

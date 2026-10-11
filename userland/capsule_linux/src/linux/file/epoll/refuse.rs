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

//! What epoll_ctl refuses before it changes the list: the descriptors, the
//! kind watched, EPOLLEXCLUSIVE, and an epoll that would watch itself.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

use super::super::epoll_rules::{exclusive_ok, loops, EPOLL_CTL_ADD};

/// The events of `fd`'s entry already in list `ep`, if there is one; or the
/// answer that refuses operation `op`, asking for `events`.
pub(super) fn refused(
    guest: &Guest,
    (ep, op, fd): (u64, u64, u64),
    events: Option<u32>,
) -> Result<Option<u32>, u64> {
    let open = |n: u64| guest.fds.get(n as usize).is_some_and(|f| f.is_open());
    if !open(ep) || !open(fd) {
        return Err(errno::fail(errno::EBADF));
    }
    let unpollable = |f: &Fd| f.kind == Kind::Device && !super::super::dev::polls(f.handle);
    let refused = |f: &Fd| matches!(f.kind, Kind::File | Kind::Dir) || unpollable(f);
    if guest.fds.get(fd as usize).is_some_and(refused) {
        return Err(errno::fail(errno::EPERM));
    }
    let Some(list) = guest.fds.get(ep as usize).filter(|f| f.kind == Kind::Epoll) else {
        return Err(errno::fail(errno::EINVAL));
    };
    let present = list.watch.iter().find(|w| w.fd == fd).map(|w| w.events);
    if fd == ep {
        return Err(errno::fail(errno::EINVAL));
    }
    let target_epoll = guest.fds.get(fd as usize).is_some_and(|f| f.kind == Kind::Epoll);
    if let Some(events) = events {
        if let Err(e) = exclusive_ok(op, events, target_epoll, present) {
            return Err(errno::fail(e));
        }
    }
    let lists = |at: u64| {
        let f = guest.fds.get(at as usize).filter(|f| f.kind == Kind::Epoll)?;
        Some(f.watch.iter().map(|w| w.fd).collect())
    };
    if op == EPOLL_CTL_ADD && target_epoll && loops(&lists, ep, fd) {
        return Err(errno::fail(errno::ELOOP));
    }
    Ok(present)
}

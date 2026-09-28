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

//! `epoll_create1` and `epoll_ctl`: the interest list a program keeps.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind, Watch};

use super::slot::install;

const EPOLL_CTL_ADD: u64 = 1;
const EPOLL_CTL_DEL: u64 = 2;
const EPOLL_CTL_MOD: u64 = 3;

/// `struct epoll_event` is packed on x86_64: a u32 of events then a u64
/// of caller data, twelve bytes and not sixteen.
pub const EVENT_LEN: usize = 12;

pub fn epoll_create(guest: &mut Guest) -> u64 {
    match install(guest, Fd::empty(Kind::Epoll)) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

/// Add, change or drop one entry, refused as Linux refuses it: a closed
/// descriptor is EBADF, a regular file or directory EPERM (always ready, so
/// never worth waiting on; Go's os.Open falls back to blocking reads on it),
/// watching the list itself EINVAL, adding twice EEXIST, changing or
/// dropping what is not there ENOENT.
pub fn epoll_ctl(guest: &mut Guest, ep: u64, op: u64, fd: u64, event: u64) -> u64 {
    let entry = match op {
        EPOLL_CTL_DEL => None,
        EPOLL_CTL_ADD | EPOLL_CTL_MOD => match read_event(guest, event) {
            Some(pair) => Some(pair),
            None => return errno::fail(errno::EFAULT),
        },
        _ => return errno::fail(errno::EINVAL),
    };
    let open = |n: u64| guest.fds.get(n as usize).is_some_and(|f| f.is_open());
    if !open(ep) || !open(fd) {
        return errno::fail(errno::EBADF);
    }
    if guest.fds.get(fd as usize).is_some_and(|f| matches!(f.kind, Kind::File | Kind::Dir)) {
        return errno::fail(errno::EPERM);
    }
    let Some(list) = guest.fds.get_mut(ep as usize).filter(|f| f.kind == Kind::Epoll) else {
        return errno::fail(errno::EINVAL);
    };
    let present = list.watch.iter().any(|w| w.fd == fd);
    match op {
        _ if fd == ep => return errno::fail(errno::EINVAL),
        EPOLL_CTL_ADD if present => return errno::fail(errno::EEXIST),
        EPOLL_CTL_MOD | EPOLL_CTL_DEL if !present => return errno::fail(errno::ENOENT),
        _ => {}
    }
    // A change re-arms the entry: it is looked at afresh, as a new one is.
    list.watch.retain(|w| w.fd != fd);
    if let Some((events, data)) = entry {
        list.watch.push(Watch::new(fd, events, data));
    }
    errno::ok(0)
}

/// Drop `fd` from every interest list, as Linux does when a descriptor is
/// closed, so a later descriptor given its number starts unregistered.
pub fn forget(guest: &mut Guest, fd: u64) {
    for list in guest.fds.iter_mut().filter(|f| f.kind == Kind::Epoll) {
        list.watch.retain(|w| w.fd != fd);
    }
}

fn read_event(guest: &Guest, at: u64) -> Option<(u32, u64)> {
    let raw = guest.read(at, EVENT_LEN)?;
    let events = u32::from_le_bytes([raw[0], raw[1], raw[2], raw[3]]);
    let mut data = [0u8; 8];
    data.copy_from_slice(&raw[4..12]);
    Some((events, u64::from_le_bytes(data)))
}

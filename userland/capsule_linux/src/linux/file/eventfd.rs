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

//! `eventfd2` and `eventfd`: a counter one side adds to and the other takes,
//! which is how one thread wakes another out of `epoll_wait`. Go's runtime
//! makes one for its poller at the first timer and throws if it cannot.

use crate::linux::abi::errno;
use crate::linux::guest::{Event, Fd, Guest, Kind};

use super::flags::{O_CLOEXEC, O_NONBLOCK};
use super::slot::install;
use crate::linux::guest::slots::pick;

/// ENFILE: no slot left in the family's table of counters.
const ENFILE: i64 = 23;

const EFD_SEMAPHORE: u64 = 1;
/// The most a counter holds. A write that would pass it waits.
pub const MOST: u64 = u64::MAX - 1;

const POLLIN: u16 = 0x001;
const POLLOUT: u16 = 0x004;
const POLLNVAL: u16 = 0x020;

pub fn eventfd2(guest: &mut Guest, initval: u64, flags: u64) -> u64 {
    if flags & !(EFD_SEMAPHORE | O_CLOEXEC | O_NONBLOCK) != 0 {
        return errno::fail(errno::EINVAL);
    }
    /* A counter no descriptor in the family names is taken again first. */
    let lent = guest.event_used.len() == guest.events.len();
    let Some(slot) = pick(guest.events.len(), |i| lent && guest.event_used.get(i) == Some(&false))
    else {
        return errno::fail(ENFILE);
    };
    // The starting value is an unsigned int.
    let event = Event { count: initval & 0xFFFF_FFFF, semaphore: flags & EFD_SEMAPHORE != 0 };
    match guest.events.get_mut(slot) {
        Some(old) => *old = event,
        None => guest.events.push(event),
    }
    if let Some(used) = guest.event_used.get_mut(slot) {
        *used = true;
    }
    let mut fd = Fd::empty(Kind::Event);
    fd.handle = slot as u32;
    fd.cloexec = flags & O_CLOEXEC != 0;
    fd.nonblock = flags & O_NONBLOCK != 0;
    match install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

/// The counter `fd` names, if it is an eventfd.
pub fn slot_of(guest: &Guest, fd: u64) -> Option<usize> {
    let entry = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Event)?;
    let slot = entry.handle as usize;
    (slot < guest.events.len()).then_some(slot)
}

/// Readable while the count is above zero, writable while one more fits.
pub fn bits(guest: &Guest, fd: u64) -> u16 {
    let Some(slot) = slot_of(guest, fd) else {
        return POLLNVAL;
    };
    let count = guest.events[slot].count;
    let readable = if count > 0 { POLLIN } else { 0 };
    readable | if count < MOST { POLLOUT } else { 0 }
}

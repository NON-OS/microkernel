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

//! Reading a timer: how many times it has fired since the last read, as a
//! u64, and whether it has fired at all, for poll and epoll.

use crate::linux::abi::errno;
use crate::linux::call::now_ms;
use crate::linux::guest::Guest;

use super::timerfd::slot_of;

const CLOCK_MONOTONIC: u64 = 1;
const POLLIN: u16 = 0x001;
const POLLNVAL: u16 = 0x020;

/// EAGAIN until it has fired; whether the caller waits is decided by who
/// called, from the descriptor's O_NONBLOCK.
pub fn read(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let Some(slot) = slot_of(guest, fd) else {
        return errno::fail(errno::EBADF);
    };
    if len < 8 {
        return errno::fail(errno::EINVAL);
    }
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    let mut timer = guest.timers[slot];
    let times = timer.take(now);
    if times == 0 {
        return errno::fail(errno::EAGAIN);
    }
    // Written before it is taken, so a bad buffer leaves the count as it was.
    if guest.write(buf, &times.to_le_bytes()) < 8 {
        return errno::fail(errno::EFAULT);
    }
    guest.timers[slot] = timer;
    errno::ok(8)
}

/// Readable once it has fired, and never writable.
pub fn bits(guest: &Guest, fd: u64) -> u16 {
    match slot_of(guest, fd) {
        Some(slot) if guest.timers[slot].fired(now_ms(CLOCK_MONOTONIC).unwrap_or(0)) => POLLIN,
        Some(_) => 0,
        None => POLLNVAL,
    }
}

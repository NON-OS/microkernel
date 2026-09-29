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

//! Reading and writing an eventfd: eight bytes, the count as a u64.
//!
//! Each answers EAGAIN where Linux would wait. Whether the caller waits
//! instead is decided by who called, from the descriptor's O_NONBLOCK.

use crate::linux::abi::errno;
use crate::linux::guest::{Event, Guest};

use super::eventfd::{slot_of, MOST};

const WORD: u64 = 8;

/// Take the whole count, or one of it under EFD_SEMAPHORE.
pub fn read(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let Some(slot) = slot_of(guest, fd) else {
        return errno::fail(errno::EBADF);
    };
    if len < WORD {
        return errno::fail(errno::EINVAL);
    }
    let Event { count, semaphore } = guest.events[slot];
    if count == 0 {
        return errno::fail(errno::EAGAIN);
    }
    let take = if semaphore { 1 } else { count };
    // Written before it is taken, so a bad buffer leaves the count as it was.
    if guest.write(buf, &take.to_le_bytes()) < WORD as i64 {
        return errno::fail(errno::EFAULT);
    }
    guest.events[slot].count = count - take;
    errno::ok(WORD)
}

/// Add to the count. All ones is refused, as Linux refuses it.
pub fn write(guest: &mut Guest, fd: u64, buf: u64, len: u64) -> u64 {
    let Some(slot) = slot_of(guest, fd) else {
        return errno::fail(errno::EBADF);
    };
    if len < WORD {
        return errno::fail(errno::EINVAL);
    }
    let Some(raw) = guest.read(buf, WORD as usize) else {
        return errno::fail(errno::EFAULT);
    };
    let add = u64::from_le_bytes(raw[..8].try_into().unwrap_or([0xFF; 8]));
    if add == u64::MAX {
        return errno::fail(errno::EINVAL);
    }
    let count = guest.events[slot].count;
    if add > MOST - count {
        return errno::fail(errno::EAGAIN);
    }
    guest.events[slot].count = count + add;
    errno::ok(WORD)
}

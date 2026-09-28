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

//! `futex`, entirely in this capsule.
//!
//! A waiter is a guest thread already parked inside its trap, so the wait
//! is the absence of a reply and the wake is the reply. The kernel needs
//! to know nothing about it.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

use super::futex_requeue::requeue;
use super::futex_time::deadline;

const FUTEX_WAIT: u64 = 0;
const FUTEX_WAKE: u64 = 1;
const FUTEX_REQUEUE: u64 = 3;
const FUTEX_CMP_REQUEUE: u64 = 4;
const FUTEX_WAIT_BITSET: u64 = 9;
const FUTEX_WAKE_BITSET: u64 = 10;
/// The operation, without FUTEX_PRIVATE_FLAG and FUTEX_CLOCK_REALTIME.
const OP_MASK: u64 = 0x7F;

/// `futex(uaddr, op, val, timeout or val2, uaddr2, val3)`. A bitset is
/// taken as matching every waiter: a wake it would not have chosen is a
/// spurious wake, which every futex caller already loops on.
pub fn futex(guest: &mut Guest, tid: u32, a: [u64; 6]) -> Answer {
    let (uaddr, op, val) = (a[0], a[1], a[2]);
    let no_bits = a[5] as u32 == 0;
    match op & OP_MASK {
        FUTEX_WAIT_BITSET | FUTEX_WAKE_BITSET if no_bits => {
            Answer::value(errno::fail(errno::EINVAL))
        }
        FUTEX_WAIT => wait(guest, tid, uaddr, val, deadline(guest, a[3], op, false)),
        FUTEX_WAIT_BITSET => wait(guest, tid, uaddr, val, deadline(guest, a[3], op, true)),
        FUTEX_WAKE | FUTEX_WAKE_BITSET => Answer::value(errno::ok(guest.wake(uaddr, val))),
        FUTEX_REQUEUE => requeue(guest, [uaddr, val, a[3], a[4]], None),
        FUTEX_CMP_REQUEUE => requeue(guest, [uaddr, val, a[3], a[4]], Some(a[5] as u32)),
        _ => Answer::value(errno::fail(errno::ENOSYS)),
    }
}

fn wait(
    guest: &mut Guest,
    tid: u32,
    uaddr: u64,
    val: u64,
    until: Result<Option<u64>, u64>,
) -> Answer {
    let until = match until {
        Ok(until) => until,
        Err(refused) => return Answer::value(refused),
    };
    let Some(seen) = super::futex_requeue::word(guest, uaddr) else {
        return Answer::value(errno::fail(errno::EFAULT));
    };
    // The word changed between the caller's own check and this one, so
    // the condition it was going to sleep on is already false.
    if u64::from(seen) != val & 0xFFFF_FFFF {
        return Answer::value(errno::fail(errno::EAGAIN));
    }
    if let Some(when) = until {
        if when <= super::now_ms(super::futex_time::CLOCK_MONOTONIC).unwrap_or(0) {
            return Answer::value(errno::fail(errno::ETIMEDOUT));
        }
        guest.futex_until.push((when, tid));
    }
    guest.waits.push((tid, uaddr));
    Answer::Park
}

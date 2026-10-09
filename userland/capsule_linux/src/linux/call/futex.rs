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
//! to know nothing about it. What the call asks for is decoded and checked
//! in `futex_op`, which the host proofs hold.

use crate::linux::abi::errno;
use crate::linux::guest::futex_pick::MATCH_ANY;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

use super::futex_op::{decode, wake_count, Cmd};
use super::futex_requeue::requeue;
use super::futex_time::deadline;

/// `futex(uaddr, op, val, timeout or val2, uaddr2, val3)`. A wait records
/// its bitset, and a wake takes only waiters whose bitset meets its own, so
/// a wake meant for one waiter never uses itself up on another.
pub fn futex(guest: &mut Guest, tid: u32, a: [u64; 6]) -> Answer {
    let (uaddr, op, val) = (a[0], a[1], a[2]);
    let cmd = match decode(uaddr, op, a[4], a[5]) {
        Ok(cmd) => cmd,
        Err(e) => return Answer::value(errno::fail(e)),
    };
    match cmd {
        Cmd::Wait { bits, absolute } => {
            wait(guest, tid, uaddr, val, bits, deadline(guest, a[3], op, absolute))
        }
        Cmd::Wake { bits } => {
            Answer::value(errno::ok(guest.wake_bits(uaddr, wake_count(val), bits)))
        }
        Cmd::Requeue { compare } => {
            requeue(guest, [uaddr, val, a[3], a[4]], compare.then_some(a[5] as u32))
        }
    }
}

fn wait(
    guest: &mut Guest,
    tid: u32,
    uaddr: u64,
    val: u64,
    bits: u32,
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
    /* A thread waits on one word at a time; anything left of an old wait goes. */
    guest.forget_futex(tid);
    if let Some(when) = until {
        if when <= super::now_ms(super::futex_time::CLOCK_MONOTONIC).unwrap_or(0) {
            return Answer::value(errno::fail(errno::ETIMEDOUT));
        }
        guest.futex_until.push((when, tid));
    }
    if bits != MATCH_ANY {
        guest.futex_bits.push((tid, bits));
    }
    guest.waits.push((tid, uaddr));
    Answer::Park
}

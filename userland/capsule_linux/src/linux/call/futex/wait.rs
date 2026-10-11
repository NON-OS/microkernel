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

//! A futex wait: refused, or the thread parked on its word.

use crate::linux::abi::errno;
use crate::linux::guest::futex_pick::MATCH_ANY;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

pub(super) fn wait(
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
    let Some(seen) = super::super::futex_requeue::word(guest, uaddr) else {
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
        if when <= super::super::now_ms(super::super::futex_time::CLOCK_MONOTONIC).unwrap_or(0) {
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

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

//! Waiting, without holding up the family.
//!
//! A sleeping guest is parked and answered when its deadline passes, so the
//! other processes and threads the personality hosts keep being served. A
//! busy wait here stopped the whole family for as long as any one slept.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

use super::clock::now_ms;

const TIMER_ABSTIME: u64 = 1;
const CLOCK_MONOTONIC: u64 = 1;
const NSEC: u64 = 1_000_000_000;

/// `nanosleep(req, rem)`.
pub fn nanosleep(guest: &mut Guest, tid: u32, req: u64) -> Answer {
    park(guest, tid, CLOCK_MONOTONIC, 0, req)
}

/// `clock_nanosleep(clock, flags, req, rem)`: relative, or until an absolute
/// time on `clock`. Errors come back as a positive errno, as Linux returns them.
pub fn clock_nanosleep(guest: &mut Guest, tid: u32, clock: u64, flags: u64, req: u64) -> Answer {
    match park(guest, tid, clock, flags, req) {
        Answer::Reply(v) if (v as i64) < 0 => Answer::Reply((v as i64).unsigned_abs()),
        other => other,
    }
}

fn park(guest: &mut Guest, tid: u32, clock: u64, flags: u64, req: u64) -> Answer {
    let Some(spec) = guest.read(req, 16) else {
        return Answer::Reply(errno::fail(errno::EFAULT));
    };
    let secs = u64::from_le_bytes(spec[..8].try_into().unwrap_or([0; 8]));
    let nanos = u64::from_le_bytes(spec[8..16].try_into().unwrap_or([0; 8]));
    let (Some(on_clock), Some(mono)) = (now_ms(clock), now_ms(CLOCK_MONOTONIC)) else {
        return Answer::Reply(errno::fail(errno::EINVAL));
    };
    if nanos >= NSEC || secs > i64::MAX as u64 {
        return Answer::Reply(errno::fail(errno::EINVAL));
    }
    let span = secs.saturating_mul(1000).saturating_add(nanos.div_ceil(1_000_000));
    // An absolute time is a distance from now on its own clock.
    let wait = if flags & TIMER_ABSTIME != 0 { span.saturating_sub(on_clock) } else { span };
    if wait == 0 {
        return Answer::Reply(errno::ok(0));
    }
    guest.sleepers.push((mono.saturating_add(wait), tid));
    Answer::Park
}

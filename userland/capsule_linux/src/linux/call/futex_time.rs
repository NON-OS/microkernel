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

//! When a futex wait gives up.
//!
//! FUTEX_WAIT takes a relative timeout. FUTEX_WAIT_BITSET takes an absolute
//! one, on CLOCK_MONOTONIC unless FUTEX_CLOCK_REALTIME is set. Every deadline
//! here is on the guest's monotonic clock, in milliseconds, rounded up.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::now_ms;

pub const CLOCK_MONOTONIC: u64 = 1;
const CLOCK_REALTIME: u64 = 0;
const FUTEX_CLOCK_REALTIME: u64 = 256;
const NSEC: i64 = 1_000_000_000;

/// None for no timeout; an errno for a timespec Linux refuses.
pub fn deadline(guest: &Guest, at: u64, op: u64, absolute: bool) -> Result<Option<u64>, u64> {
    if at == 0 {
        return Ok(None);
    }
    let Some(spec) = guest.read(at, 16) else {
        return Err(errno::fail(errno::EFAULT));
    };
    let secs = i64::from_le_bytes(spec[..8].try_into().unwrap_or([0; 8]));
    let nanos = i64::from_le_bytes(spec[8..16].try_into().unwrap_or([0; 8]));
    if secs < 0 || !(0..NSEC).contains(&nanos) {
        return Err(errno::fail(errno::EINVAL));
    }
    let span =
        (secs as u64).saturating_mul(1000).saturating_add((nanos as u64).div_ceil(1_000_000));
    let mono = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    Ok(Some(match (absolute, op & FUTEX_CLOCK_REALTIME != 0) {
        (false, _) => mono.saturating_add(span),
        (true, false) => span,
        // A wall-clock time is a distance from now on the wall clock.
        (true, true) => {
            mono.saturating_add(span.saturating_sub(now_ms(CLOCK_REALTIME).unwrap_or(0)))
        }
    }))
}

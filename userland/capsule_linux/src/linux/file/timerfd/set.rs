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

//! `timerfd_settime`: a timer one-shot or periodic, set relative to now or
//! to an absolute time on its clock. `timerfd_gettime` is get.rs.

use crate::linux::abi::errno;
use crate::linux::call::now_ms;
use crate::linux::guest::Guest;

use super::get::current;
use super::open::slot_of;
use super::spec::{read_spec, write_spec};

const CLOCK_MONOTONIC: u64 = 1;
const TFD_TIMER_ABSTIME: u64 = 1;
const TFD_TIMER_CANCEL_ON_SET: u64 = 2;

/// Arm or disarm, writing the previous setting to `old` when it is named.
pub fn timerfd_settime(guest: &mut Guest, fd: u64, flags: u64, new: u64, old: u64) -> u64 {
    let Some(slot) = slot_of(guest, fd) else {
        return errno::fail(errno::EBADF);
    };
    if flags & !(TFD_TIMER_ABSTIME | TFD_TIMER_CANCEL_ON_SET) != 0 {
        return errno::fail(errno::EINVAL);
    }
    let (every, value) = match read_spec(guest, new) {
        Ok(pair) => pair,
        Err(refused) => return refused,
    };
    if old != 0 && write_spec(guest, old, current(guest, slot)) < 0 {
        return errno::fail(errno::EFAULT);
    }
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    let timer = &mut guest.timers[slot];
    timer.every = every;
    timer.due = match (value, flags & TFD_TIMER_ABSTIME != 0) {
        (0, _) => 0,
        (span, false) => now.saturating_add(span),
        // An absolute time is a distance from now on the timer's own clock.
        (at, true) => {
            let on_clock = now_ms(timer.clock).unwrap_or(now);
            now.saturating_add(at.saturating_sub(on_clock)).max(1)
        }
    };
    errno::ok(0)
}

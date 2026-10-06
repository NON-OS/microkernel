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

//! timer_settime: arm a timer for a relative time, or an absolute one on its
//! own clock, repeating every interval; a zero value disarms it. The old
//! setting is written first when the caller asked for it.

use super::signal_its::read_spec;
use super::timer_ops::{find, timer_gettime};
use crate::linux::abi::errno;
use crate::linux::call::now_ms;
use crate::linux::guest::Guest;

const TIMER_ABSTIME: u64 = 1;
const CLOCK_MONOTONIC: u64 = 1;

pub fn timer_settime(guest: &mut Guest, id: u64, flags: u64, new: u64, old: u64) -> u64 {
    let Some(at) = find(guest, id) else {
        return errno::fail(errno::EINVAL);
    };
    if new == 0 {
        return errno::fail(errno::EINVAL);
    }
    let (interval, value) = match read_spec(guest, new) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let rc = timer_gettime(guest, id, old);
    let t = &mut guest.signals.timers[at];
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    let wait = match flags & TIMER_ABSTIME {
        0 => value,
        _ => value.saturating_sub(now_ms(t.clock).unwrap_or(0)),
    };
    t.due = (value != 0).then(|| now.saturating_add(wait));
    t.interval = if value == 0 { 0 } else { interval };
    t.overrun = 0;
    if old != 0 {
        rc
    } else {
        errno::ok(0)
    }
}

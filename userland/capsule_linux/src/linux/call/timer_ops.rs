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

//! timer_gettime, timer_getoverrun and timer_delete; timer_settime is in
//! timer_set. Expiry raises the timer's signal with SI_TIMER, its id and
//! sigev_value; an expiry while that signal still waits counts as an overrun,
//! reported by the signal and by timer_getoverrun.

use crate::linux::abi::errno;
use crate::linux::call::now_ms;
use crate::linux::guest::Guest;

use super::signal_its::write_spec;

const CLOCK_MONOTONIC: u64 = 1;

pub fn timer_gettime(guest: &mut Guest, id: u64, out: u64) -> u64 {
    let Some(at) = find(guest, id) else {
        return errno::fail(errno::EINVAL);
    };
    let t = guest.signals.timers[at];
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    let left = t.due.map_or(0, |d| d.saturating_sub(now).max(1));
    write_spec(guest, out, t.interval, left)
}

pub fn timer_getoverrun(guest: &mut Guest, id: u64) -> u64 {
    match find(guest, id) {
        Some(at) => errno::ok(guest.signals.timers[at].last_overrun as u64),
        None => errno::fail(errno::EINVAL),
    }
}

pub fn timer_delete(guest: &mut Guest, id: u64) -> u64 {
    let Some(at) = find(guest, id) else {
        return errno::fail(errno::EINVAL);
    };
    let t = guest.signals.timers.remove(at);
    guest.signals.drop_timer_signal(t.id);
    errno::ok(0)
}

pub fn find(guest: &Guest, id: u64) -> Option<usize> {
    let id = i32::try_from(id).ok()?;
    guest.signals.timers.iter().position(|t| t.id == id)
}

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

//! `timerfd_create`, `timerfd_settime` and `timerfd_gettime`, as Linux
//! defines them: a timer on a named clock, one-shot or periodic, set
//! relative to now or to an absolute time on its clock.

use crate::linux::abi::errno;
use crate::linux::call::now_ms;
use crate::linux::guest::{Fd, Guest, Kind, Timer};

use super::flags::{O_CLOEXEC, O_NONBLOCK};
use super::slot::install;
use super::timerfd_spec::{read_spec, write_spec};
use crate::linux::guest::slots::pick;

const CLOCK_MONOTONIC: u64 = 1;
/// REALTIME, MONOTONIC, BOOTTIME, REALTIME_ALARM, BOOTTIME_ALARM.
const CLOCKS: [u64; 5] = [0, 1, 7, 8, 9];
const TFD_TIMER_ABSTIME: u64 = 1;
const TFD_TIMER_CANCEL_ON_SET: u64 = 2;
/// ENFILE: no slot left in the family's table of timers.
const ENFILE: i64 = 23;

pub fn timerfd_create(guest: &mut Guest, clock: u64, flags: u64) -> u64 {
    if !CLOCKS.contains(&clock) || flags & !(O_NONBLOCK | O_CLOEXEC) != 0 {
        return errno::fail(errno::EINVAL);
    }
    /* A timer no descriptor in the family names is taken again first. */
    let lent = guest.timer_used.len() == guest.timers.len();
    let Some(slot) = pick(guest.timers.len(), |i| lent && guest.timer_used.get(i) == Some(&false))
    else {
        return errno::fail(ENFILE);
    };
    let timer = Timer { clock, ..Timer::default() };
    match guest.timers.get_mut(slot) {
        Some(old) => *old = timer,
        None => guest.timers.push(timer),
    }
    if let Some(used) = guest.timer_used.get_mut(slot) {
        *used = true;
    }
    let mut fd = Fd::empty(Kind::Timer);
    fd.handle = slot as u32;
    fd.nonblock = flags & O_NONBLOCK != 0;
    fd.cloexec = flags & O_CLOEXEC != 0;
    match install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}

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

pub fn timerfd_gettime(guest: &mut Guest, fd: u64, out: u64) -> u64 {
    let Some(slot) = slot_of(guest, fd) else {
        return errno::fail(errno::EBADF);
    };
    match write_spec(guest, out, current(guest, slot)) < 0 {
        true => errno::fail(errno::EFAULT),
        false => errno::ok(0),
    }
}

/// The interval, and what is left before the next firing.
fn current(guest: &Guest, slot: usize) -> (u64, u64) {
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    let timer = guest.timers[slot];
    let left = if timer.due == 0 { 0 } else { timer.due.saturating_sub(now).max(1) };
    (timer.every, left)
}

/// The timer `fd` names, if it is a timerfd.
pub fn slot_of(guest: &Guest, fd: u64) -> Option<usize> {
    let entry = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Timer)?;
    let slot = entry.handle as usize;
    (slot < guest.timers.len()).then_some(slot)
}

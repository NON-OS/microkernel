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

//! `timerfd_create`, as Linux defines it: a timer on a named clock, set
//! and read in set.rs.

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind, Timer};

use super::super::flags::{O_CLOEXEC, O_NONBLOCK};
use super::super::slot::install;
use crate::linux::guest::slots::pick;

/// REALTIME, MONOTONIC, BOOTTIME, REALTIME_ALARM, BOOTTIME_ALARM.
const CLOCKS: [u64; 5] = [0, 1, 7, 8, 9];
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

/// The timer `fd` names, if it is a timerfd.
pub fn slot_of(guest: &Guest, fd: u64) -> Option<usize> {
    let entry = guest.fds.get(fd as usize).filter(|f| f.kind == Kind::Timer)?;
    let slot = entry.handle as usize;
    (slot < guest.timers.len()).then_some(slot)
}

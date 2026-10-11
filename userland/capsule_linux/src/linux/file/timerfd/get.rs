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

//! `timerfd_gettime`: what a timer's interval is, and how long is left.

use crate::linux::abi::errno;
use crate::linux::call::now_ms;
use crate::linux::guest::Guest;

use super::open::slot_of;
use super::spec::write_spec;

const CLOCK_MONOTONIC: u64 = 1;

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
pub(super) fn current(guest: &Guest, slot: usize) -> (u64, u64) {
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    let timer = guest.timers[slot];
    let left = if timer.due == 0 { 0 } else { timer.due.saturating_sub(now).max(1) };
    (timer.every, left)
}

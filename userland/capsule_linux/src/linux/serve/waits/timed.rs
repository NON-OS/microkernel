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

//! epoll_wait, poll, ppoll, select and pselect6: tried now, else parked until
//! something they watch is ready or their deadline.

use super::super::answer::Answer;
use super::super::waits_mask;
use super::super::waits_time::span;
use super::super::waits_try::{attempt, expire};
use super::io::park;
use crate::linux::call::now_ms;
use crate::linux::guest::{Blocked, Guest};

const CLOCK_MONOTONIC: u64 = 1;

/// The epoll, poll and select calls: each waits until something it watches
/// is ready or its timeout passes, and a timeout of zero only looks.
pub fn timed(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> Answer {
    let limit = match span(guest, nr, &a) {
        Ok(limit) => limit,
        Err(refused) => return Answer::value(refused),
    };
    /* ppoll, pselect6 and epoll_pwait wait under a mask of their own. */
    let mask = match waits_mask::read(guest, nr, &a) {
        Ok(mask) => mask,
        Err(refused) => return Answer::value(refused),
    };
    if let Some(mask) = mask {
        waits_mask::enter(guest, tid, mask);
    }
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    let deadline = limit.map(|ms| now.saturating_add(ms));
    let mut wait = Blocked { tid, nr, args: a, deadline, done: 0 };
    let value = match attempt(guest, &mut wait) {
        Some(v) => v,
        None if deadline.is_some_and(|d| d <= now) => expire(guest, &wait),
        None => return park(guest, wait),
    };
    waits_mask::leave(guest, tid, nr);
    Answer::value(value)
}

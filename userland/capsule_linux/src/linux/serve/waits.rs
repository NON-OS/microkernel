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

//! Calls that wait for a descriptor: `epoll_wait`, `poll`, `ppoll`,
//! `select` and `pselect6` until their timeout, and a read or write that
//! would block on a pipe or an eventfd.
//!
//! Each is tried when it arrives. One that cannot complete is left parked
//! in its trap, and the family tries it again after every answer and at its
//! deadline (`family_waits`), so the other threads and processes it hosts
//! keep being served while it waits.

use crate::linux::abi::errno;
use crate::linux::call::now_ms;
use crate::linux::guest::{Blocked, Guest};

use super::answer::Answer;
use super::waits_time::span;
use super::waits_try::{attempt, expire};

const CLOCK_MONOTONIC: u64 = 1;

/// The epoll, poll and select calls: each waits until something it watches
/// is ready or its timeout passes, and a timeout of zero only looks.
pub fn timed(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> Answer {
    let limit = match span(guest, nr, &a) {
        Ok(limit) => limit,
        Err(refused) => return Answer::value(refused),
    };
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    let deadline = limit.map(|ms| now.saturating_add(ms));
    let wait = Blocked { tid, nr, args: a, deadline };
    match attempt(guest, &wait) {
        Some(v) => Answer::value(v),
        None if deadline.is_some_and(|d| d <= now) => Answer::value(expire(guest, &wait)),
        None => park(guest, wait),
    }
}

/// A read or write that answers EAGAIN waits instead, unless its descriptor
/// is non-blocking.
pub fn io(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> Answer {
    let wait = Blocked { tid, nr, args: a, deadline: None };
    match attempt(guest, &wait) {
        Some(v) => Answer::value(v),
        None if guest.fds.get(a[0] as usize).is_some_and(|f| f.nonblock) => {
            Answer::value(errno::fail(errno::EAGAIN))
        }
        None => park(guest, wait),
    }
}

fn park(guest: &mut Guest, wait: Blocked) -> Answer {
    guest.blocked.push(wait);
    Answer::Park
}

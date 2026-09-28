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

//! Calls that wait for a descriptor: `epoll_wait` until its timeout, and
//! a read or write that would block on an eventfd or a full pipe.
//!
//! Each is tried when it arrives. One that cannot complete is left parked
//! in its trap, and the family tries it again after every answer and at its
//! deadline (`family_waits`), so the other threads and processes it hosts
//! keep being served while it waits.

use crate::linux::abi::{errno, nr};
use crate::linux::call::{self, now_ms};
use crate::linux::file;
use crate::linux::guest::{Blocked, Guest, Kind};

use super::answer::Answer;

const CLOCK_MONOTONIC: u64 = 1;

/// `epoll_wait` and `epoll_pwait`: the timeout is an int of milliseconds,
/// and a negative one waits until something is ready.
pub fn epoll(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> Answer {
    let now = now_ms(CLOCK_MONOTONIC).unwrap_or(0);
    let deadline = u64::try_from(a[3] as u32 as i32).ok().map(|ms| now.saturating_add(ms));
    let wait = Blocked { tid, nr, args: a, deadline };
    match attempt(guest, &wait) {
        Some(v) => Answer::value(v),
        None if deadline.is_some_and(|d| d <= now) => Answer::value(0),
        None => park(guest, wait),
    }
}

/// True for the reads and writes that can wait: an eventfd either way, and
/// a pipe written to, whose reads already wait on their own.
pub fn may_wait(guest: &Guest, nr: u64, fd: u64) -> bool {
    match guest.fds.get(fd as usize).map(|f| f.kind) {
        Some(Kind::Event) => true,
        Some(Kind::Pipe) => nr == nr::WRITE,
        _ => false,
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

/// The call's answer if it can complete now, None if it would wait.
pub fn attempt(guest: &mut Guest, wait: &Blocked) -> Option<u64> {
    let a = wait.args;
    let again = errno::fail(errno::EAGAIN);
    match wait.nr {
        nr::READ => Some(call::read(guest, a[0], a[1], a[2])).filter(|&v| v != again),
        nr::WRITE => Some(call::write(guest, a[0], a[1], a[2])).filter(|&v| v != again),
        _ => Some(file::epoll_wait(guest, a[0], a[1], a[2])).filter(|&v| v != 0),
    }
}

fn park(guest: &mut Guest, wait: Blocked) -> Answer {
    guest.blocked.push(wait);
    Answer::Park
}

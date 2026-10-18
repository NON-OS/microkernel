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

//! A read or write that would block, parked unless non-blocking, and the
//! parking itself.

use super::super::answer::Answer;
use super::super::waits_try::attempt;
use crate::linux::abi::errno;
use crate::linux::guest::{Blocked, Guest};

/// A read or write that answers EAGAIN waits instead, unless its descriptor
/// is non-blocking.
pub fn io(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> Answer {
    let mut wait = Blocked { tid, nr, args: a, deadline: None, done: 0 };
    match attempt(guest, &mut wait) {
        Some(v) => Answer::value(v),
        None if guest.fds.get(a[0] as usize).is_some_and(|f| f.nonblock) => {
            Answer::value(errno::fail(errno::EAGAIN))
        }
        None => park(guest, wait),
    }
}

pub(super) fn park(guest: &mut Guest, wait: Blocked) -> Answer {
    guest.blocked.push(wait);
    Answer::Park
}

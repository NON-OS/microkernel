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

//! FUTEX_REQUEUE and FUTEX_CMP_REQUEUE: wake some waiters on one word and
//! move more of them to wait on another, as musl's condition variables do
//! to hand their waiters to the mutex.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// `[uaddr, wake, move, uaddr2]`; `expect` is CMP_REQUEUE's val3. Plain
/// requeue answers how many it woke, the compare form how many it woke or
/// moved.
pub fn requeue(guest: &mut Guest, a: [u64; 4], expect: Option<u32>) -> Answer {
    let [from, wake, moved, to] = a;
    let (wake, moved) = (wake as u32 as i32, moved as u32 as i32);
    if wake < 0 || moved < 0 {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    if let Some(want) = expect {
        match word(guest, from) {
            None => return Answer::value(errno::fail(errno::EFAULT)),
            Some(seen) if seen != want => return Answer::value(errno::fail(errno::EAGAIN)),
            Some(_) => {}
        }
    }
    let woken = guest.wake(from, wake as u64);
    let mut shifted = 0u64;
    for w in guest.waits.iter_mut().filter(|w| w.1 == from).take(moved as usize) {
        w.1 = to;
        shifted += 1;
    }
    Answer::value(errno::ok(if expect.is_some() { woken + shifted } else { woken }))
}

/// The futex word at `uaddr`.
pub fn word(guest: &Guest, uaddr: u64) -> Option<u32> {
    let bytes = guest.read(uaddr, 4)?;
    Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
}

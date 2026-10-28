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

//! FUTEX_WAKE_OP: change a second word, wake waiters on the first, and wake
//! waiters on the second too when its old value passes the test.
//!
//! Qt's QSemaphore releases with it on a 64-bit machine and checks no answer,
//! so while it was refused a release woke nobody: a blocking queued call,
//! which is how Qt's D-Bus connects to the session bus on its own thread,
//! left the calling thread asleep for good.

use super::super::futex_op::{decode_op, wake_count};
use super::super::futex_requeue::word;
use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// `futex(uaddr, FUTEX_WAKE_OP, wake, wake2, uaddr2, op)`, answering how many
/// it woke on both words.
pub(super) fn wake_op(guest: &mut Guest, a: [u64; 6]) -> Answer {
    let (uaddr, uaddr2) = (a[0], a[4]);
    let op = match decode_op(a[5]) {
        Ok(op) => op,
        Err(e) => return Answer::value(errno::fail(e)),
    };
    if !uaddr.is_multiple_of(4) || !uaddr2.is_multiple_of(4) {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    let Some(old) = word(guest, uaddr2) else {
        return Answer::value(errno::fail(errno::EFAULT));
    };
    /*
     * Linux changes the word atomically. Here it is read and written back,
     * and only when the operation changes it, so the OR of zero Qt asks for
     * never writes over a store another thread makes meanwhile.
     */
    let new = op.apply(old);
    if new != old && guest.write(uaddr2, &new.to_le_bytes()) != 4 {
        return Answer::value(errno::fail(errno::EFAULT));
    }
    let mut woken = guest.wake(uaddr, wake_count(a[2]));
    if op.holds(old) {
        woken += guest.wake(uaddr2, wake_count(a[3]));
    }
    Answer::value(errno::ok(woken))
}

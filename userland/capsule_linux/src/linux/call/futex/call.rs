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

//! The futex call, its operation decoded and answered.

use super::super::futex_op::{decode, wake_count, Cmd};
use super::super::futex_requeue::requeue;
use super::super::futex_time::deadline;
use super::wait::wait;
use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// `futex(uaddr, op, val, timeout or val2, uaddr2, val3)`. A wait records
/// its bitset, and a wake takes only waiters whose bitset meets its own, so
/// a wake meant for one waiter never uses itself up on another.
pub fn futex(guest: &mut Guest, tid: u32, a: [u64; 6]) -> Answer {
    let (uaddr, op, val) = (a[0], a[1], a[2]);
    let cmd = match decode(uaddr, op, a[4], a[5]) {
        Ok(cmd) => cmd,
        Err(e) => return Answer::value(errno::fail(e)),
    };
    match cmd {
        Cmd::Wait { bits, absolute } => {
            wait(guest, tid, uaddr, val, bits, deadline(guest, a[3], op, absolute))
        }
        Cmd::Wake { bits } => {
            Answer::value(errno::ok(guest.wake_bits(uaddr, wake_count(val), bits)))
        }
        Cmd::Requeue { compare } => {
            requeue(guest, [uaddr, val, a[3], a[4]], compare.then_some(a[5] as u32))
        }
    }
}

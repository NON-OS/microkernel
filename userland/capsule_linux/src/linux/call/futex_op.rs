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

//! What a futex call asks for, decoded and checked as Linux's do_futex
//! checks it before any word is read, and the counts it carries as Linux
//! reads them. Pure, so the host proofs hold every refusal.

use crate::linux::abi::errno;
use crate::linux::guest::futex_pick::MATCH_ANY;

const FUTEX_WAIT: u64 = 0;
const FUTEX_WAKE: u64 = 1;
const FUTEX_REQUEUE: u64 = 3;
const FUTEX_CMP_REQUEUE: u64 = 4;
const FUTEX_WAIT_BITSET: u64 = 9;
const FUTEX_WAKE_BITSET: u64 = 10;
const FUTEX_PRIVATE_FLAG: u64 = 128;
const FUTEX_CLOCK_REALTIME: u64 = 256;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cmd {
    /// Wait while the word holds the value, to be woken by a wake whose
    /// bitset meets `bits`. WAIT_BITSET's timeout is a deadline, WAIT's a
    /// distance from now.
    Wait { bits: u32, absolute: bool },
    /// Wake waiters on the word whose bitset meets `bits`.
    Wake { bits: u32 },
    /// Wake some waiters and move more to a second word; `compare` for
    /// CMP_REQUEUE, which first checks the word holds val3.
    Requeue { compare: bool },
}

/// `futex(uaddr, op, val, timeout or val2, uaddr2, val3)`'s command, or the
/// errno Linux refuses it with: FUTEX_CLOCK_REALTIME on anything but
/// WAIT_BITSET, and any operation not served, ENOSYS; a zero bitset, and a
/// word not on a four-byte boundary, EINVAL.
pub fn decode(uaddr: u64, op: u64, uaddr2: u64, val3: u64) -> Result<Cmd, i64> {
    let cmd = op & !(FUTEX_PRIVATE_FLAG | FUTEX_CLOCK_REALTIME);
    if op & FUTEX_CLOCK_REALTIME != 0 && cmd != FUTEX_WAIT_BITSET {
        return Err(errno::ENOSYS);
    }
    let bits = val3 as u32;
    let got = match cmd {
        FUTEX_WAIT => Cmd::Wait { bits: MATCH_ANY, absolute: false },
        FUTEX_WAIT_BITSET => Cmd::Wait { bits, absolute: true },
        FUTEX_WAKE => Cmd::Wake { bits: MATCH_ANY },
        FUTEX_WAKE_BITSET => Cmd::Wake { bits },
        FUTEX_REQUEUE => Cmd::Requeue { compare: false },
        FUTEX_CMP_REQUEUE => Cmd::Requeue { compare: true },
        _ => return Err(errno::ENOSYS),
    };
    if matches!(got, Cmd::Wait { bits: 0, .. } | Cmd::Wake { bits: 0 }) {
        return Err(errno::EINVAL);
    }
    let requeues = matches!(got, Cmd::Requeue { .. });
    if !uaddr.is_multiple_of(4) || (requeues && !uaddr2.is_multiple_of(4)) {
        return Err(errno::EINVAL);
    }
    Ok(got)
}

/// How many waiters a wake of `val` takes. The count is an int, and Linux
/// wakes one before it compares, so zero or a negative count takes one.
pub fn wake_count(val: u64) -> u64 {
    (val as u32 as i32).max(1) as u64
}

/// A requeue's two counts, ints both, EINVAL when either is negative.
pub fn requeue_counts(wake: u64, moved: u64) -> Result<(u64, u64), i64> {
    match (wake as u32 as i32, moved as u32 as i32) {
        (w, m) if w < 0 || m < 0 => Err(errno::EINVAL),
        (w, m) => Ok((w as u64, m as u64)),
    }
}

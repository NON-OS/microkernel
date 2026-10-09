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

//! How many tasks a family may hold at once. Pure, so the host proofs hold
//! it.
//!
//! Every process and thread of a family is a kernel process, made by the
//! kernel for this capsule with nothing to stop the next, and a record in
//! this capsule's memory, the process's copy of its region list included.
//! A fork bomb in a guest therefore ran the machine's process table and
//! this capsule's heap out for everything else. Linux bounds it with
//! RLIMIT_NPROC, refusing fork and clone with EAGAIN at the limit, and a
//! zombie not yet waited for counts as the process it still is; so does
//! this, and getrlimit reports the same number.

use crate::linux::abi::errno;

/// RLIMIT_NPROC, soft and hard.
pub const MAX_TASKS: usize = 512;

/// The tasks a family holds: each process, its other threads, and its
/// children that have ended and are not yet waited for.
pub fn count(each: impl Iterator<Item = (usize, usize)>) -> usize {
    each.map(|(threads, zombies)| threads.saturating_add(zombies).saturating_add(1))
        .fold(0, usize::saturating_add)
}

/// Whether one more task may be made beside `tasks`: EAGAIN at the limit.
pub fn room(tasks: usize) -> Result<(), i64> {
    match tasks < MAX_TASKS {
        true => Ok(()),
        false => Err(errno::EAGAIN),
    }
}

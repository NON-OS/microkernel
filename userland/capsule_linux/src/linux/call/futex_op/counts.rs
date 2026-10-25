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

//! The counts a futex wake or requeue carries, as Linux reads them. Pure, so
//! the host proofs hold them.

use crate::linux::abi::errno;

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

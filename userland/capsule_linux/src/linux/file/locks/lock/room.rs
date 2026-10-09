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

/*
 * How many locks the family's table holds. Pure, so the host proofs hold
 * it.
 *
 * Every lock is a record in this capsule's memory, and a record lock is
 * split, never merged: a guest locking every other byte of a file made one
 * record a call, each walked by every later call, with nothing to stop it
 * short of the heap. Linux answers ENOLCK when its lock table is full,
 * and so does this one past MOST_LOCKS, far more than any program holds.
 */

use crate::linux::abi::errno;

pub const MOST_LOCKS: usize = 4096;

/*
 * Whether a change that leaves `after` locks where there were `before` is
 * taken: ENOLCK when it grows the table past MOST_LOCKS. One that does
 * not grow it, an unlock or a lock replacing its owner's own, always is.
 */
pub fn fits(before: usize, after: usize) -> Result<(), i64> {
    match after > MOST_LOCKS && after > before {
        true => Err(errno::ENOLCK),
        false => Ok(()),
    }
}

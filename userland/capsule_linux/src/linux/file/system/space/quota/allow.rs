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

/* Which changes the quota takes, to the store and to the family's copies. */

use super::super::super::declared::{HOME, HOMES, HOME_NAMES, PRIVATE, PRIVATE_NAMES};
use super::kept::Kept;
use crate::linux::abi::errno;

/*
 * Whether a change from `now` to `then` is taken: ENOSPC when it makes
 * either count larger and leaves it past its ceiling, as a full tmpfs
 * answers a write or a new name. A change that makes neither larger is
 * always taken, so a family at or over its quota can still shrink a file
 * or remove a name.
 */
pub fn allows(now: Kept, then: Kept) -> Result<(), i64> {
    held_to(now, then, PRIVATE, PRIVATE_NAMES)
}

/*
 * The same for the family's home, `all` the bytes every home keeps before
 * and after: past its own ceiling, or past what all homes may keep.
 */
pub fn home_allows(now: Kept, then: Kept, all: (u64, u64)) -> Result<(), i64> {
    held_to(now, then, HOME, HOME_NAMES)?;
    held_to(Kept { bytes: all.0, names: 0 }, Kept { bytes: all.1, names: 0 }, HOMES, 0)
}

fn held_to(now: Kept, then: Kept, bytes: u64, names: u64) -> Result<(), i64> {
    let grows_past = |was: u64, will: u64, most: u64| will > most && will > was;
    if grows_past(now.bytes, then.bytes, bytes) || grows_past(now.names, then.names, names) {
        return Err(errno::ENOSPC);
    }
    Ok(())
}

/*
 * Whether one of the family's copies may go from `was` bytes to `len`,
 * when the copies hold `total` bytes together. The copies are this
 * capsule's own memory, and a family that writes many files and closes
 * none puts nothing in the store for the quota to see, so the copies
 * together are held to the same ceiling: no number of files open at once
 * holds more of this capsule than the store would take of them.
 */
pub fn copies_allow(total: u64, was: u64, len: u64) -> Result<(), i64> {
    let then = total.saturating_sub(was).saturating_add(len);
    allows(Kept { bytes: total, names: 0 }, Kept { bytes: then, names: 0 })
}

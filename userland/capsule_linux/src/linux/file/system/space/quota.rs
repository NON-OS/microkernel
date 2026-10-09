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
 * The family's quota on its private directories, apart from the store
 * and the calls that ask: what may be kept, and which changes it refuses.
 * Pure, so the host proofs hold it.
 */

use crate::linux::abi::errno;

use super::super::declared::{PRIVATE, PRIVATE_NAMES};

/* What the family keeps there: bytes, and names, a file or directory each. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Kept {
    pub bytes: u64,
    pub names: u64,
}

impl Kept {
    /* `self` with `more` added, saturating: a count can only be refused. */
    pub fn plus(self, more: Kept) -> Kept {
        Kept {
            bytes: self.bytes.saturating_add(more.bytes),
            names: self.names.saturating_add(more.names),
        }
    }
}

/*
 * Whether a change from `now` to `then` is taken: ENOSPC when it makes
 * either count larger and leaves it past its ceiling, as a full tmpfs
 * answers a write or a new name. A change that makes neither larger is
 * always taken, so a family at or over its quota can still shrink a file
 * or remove a name.
 */
pub fn allows(now: Kept, then: Kept) -> Result<(), i64> {
    let grows_past = |was: u64, will: u64, most: u64| will > most && will > was;
    if grows_past(now.bytes, then.bytes, PRIVATE)
        || grows_past(now.names, then.names, PRIVATE_NAMES)
    {
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

/* The names left before the next one is refused, as statfs's f_ffree. */
pub fn names_free(now: Kept) -> u64 {
    PRIVATE_NAMES.saturating_sub(now.names)
}

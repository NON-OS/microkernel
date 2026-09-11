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

//! Replacing a payload of the same length, in place.
//!
//! The appender writes a name once. That is right for capsule images, which
//! are written by the packer and never edited, and wrong for the one kind of
//! file a running system rewrites: a record it keeps for itself. The wallet's
//! sealed vault is written whenever the wallet changes, and its second write
//! was refused with `Exists`, so a machine could keep the first wallet it ever
//! had and no other.
//!
//! Nothing here allocates or moves an extent. The replacement must be exactly
//! as long as what is there, which every fixed-format record is, and then the
//! payload is overwritten where it lies and the table of contents keeps the
//! same offset and length with a new digest. A different length is refused,
//! because moving an extent needs an allocator this container does not have
//! and reporting success without one would leave the old bytes on the disk.

use super::digest::digest16;
use super::error::BlkError;
use super::store_patch::patch_digest;
use super::store_rules::permitted;
use super::store_toc::TocEntry;
use super::store_write::{commit, write_payload};

/// Overwrite `entry` with `data`, which must be the same length.
///
/// `toc` is the table region as read, and `index` the entry's position in
/// it, because the digest is written back into that record and nothing else
/// in the region changes: the count is the same and every other entry keeps
/// its offset.
pub fn replace(toc: &[u8], index: usize, entry: &TocEntry, data: &[u8]) -> Result<(), BlkError> {
    permitted(&entry.name, entry.len, data.len())?;
    let region = patch_digest(toc, index, &digest16(data))?;
    write_payload(entry.offset, data)?;
    commit(&region)
}

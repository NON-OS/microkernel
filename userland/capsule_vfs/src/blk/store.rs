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

// Pulls the packaged files off the block device. Payloads run to hundreds of
// kilobytes while the driver caps a request at MAX_READ_BYTES, so every extent
// is walked in whole-sector chunks and trimmed back to its declared length.
use alloc::string::String;
use alloc::vec::Vec;

use nonos_disk_map::digest16;
use nonos_libc::mk_debug;

use super::streamed::Extent;
use super::error::BlkError;
use super::store_toc::TocEntry;
use super::wire::SECTOR_SIZE;

pub struct StoreEntry {
    pub name: String,
    pub data: Vec<u8>,
    /// Set for an entry served from the device (`streamed`); `data` is then
    /// empty.
    pub streamed: Option<Extent>,
}

/*
 * The disk map's, which mk/40-run.mk also passes to nonos-store-pack as
 * `--lba` and the installer writes the store at.
 */
pub(super) use nonos_disk_map::STORE_BASE_LBA;

/// Verify a payload and turn it into a staged entry.
///
/// Shared with the resumable loader so both paths check the digest the same
/// way. A second copy of this is how one of them ends up trusting bytes the
/// other would have refused.
pub(super) fn finish_entry(entry: &TocEntry, data: Vec<u8>) -> Result<StoreEntry, BlkError> {
    verify(&entry.digest, &entry.name, &data)?;
    Ok(StoreEntry { name: entry.name.clone(), data, streamed: None })
}

/// An entry served from the device: its place is kept, none of its bytes.
pub(super) fn stream_entry(entry: &TocEntry) -> StoreEntry {
    let extent = Extent { offset: entry.offset, len: entry.len };
    StoreEntry { name: entry.name.clone(), data: Vec::new(), streamed: Some(extent) }
}

fn verify(digest: &[u8; 16], name: &str, data: &[u8]) -> Result<(), BlkError> {
    if *digest == [0u8; 16] {
        return Ok(());
    }
    if digest16(data) == *digest {
        mark(b"[PKG] vfy ok ", name);
        Ok(())
    } else {
        mark(b"[PKG] vfy FAIL ", name);
        Err(BlkError::BadContainer)
    }
}

fn mark(tag: &[u8], name: &str) {
    let mut line = Vec::with_capacity(tag.len() + name.len() + 1);
    line.extend_from_slice(tag);
    line.extend_from_slice(name.as_bytes());
    line.push(b'\n');
    let _ = mk_debug(line.as_ptr(), line.len());
}

pub(super) fn sector_span(bytes: usize) -> usize {
    bytes.div_ceil(SECTOR_SIZE) * SECTOR_SIZE
}

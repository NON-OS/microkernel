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

//! One small file out of an installed disk's own store, which is not
//! encrypted, read the way vfs reads the store at boot
//! (`userland/capsule_vfs/src/blk/store_header.rs`, `store_toc.rs`,
//! `store.rs`): the header's magic, version and count, then the descriptor
//! of that name, its extent held to the store's window past the table the
//! count needs, and its payload to its digest, which zero stands in for.

use alloc::vec::Vec;

use nonos_disk_map::{digest16, ENTRY_LEN, HEADER_LEN, MAX_ENTRIES, NAME_LEN, SECTOR_SIZE};
use nonos_disk_map::{STORE_BASE_LBA, STORE_END_LBA, STORE_MAGIC, STORE_VERSION, TOC_SPAN};

use crate::boot_media::read::{le32, le64, sectors};
use crate::sink::BlockSink;

/// The most a file read this way may hold: setup's answers are a few
/// hundred bytes, and nothing larger is read from a disk being updated.
const MOST: u64 = 4096;

pub fn kept_file(disk: &mut dyn BlockSink, name: &str) -> Option<Vec<u8>> {
    let sector = SECTOR_SIZE as u64;
    let toc = sectors(disk, STORE_BASE_LBA, TOC_SPAN as u64 / sector)?;
    let count = le32(&toc, 12) as usize;
    let header = toc[..8] == STORE_MAGIC && le32(&toc, 8) == STORE_VERSION as u64;
    if !header || count > MAX_ENTRIES {
        return None;
    }
    let table = (HEADER_LEN + ENTRY_LEN * count).div_ceil(SECTOR_SIZE) as u64 * sector;
    let floor = STORE_BASE_LBA * sector + table;
    let entries = toc[HEADER_LEN..HEADER_LEN + ENTRY_LEN * count].chunks_exact(ENTRY_LEN);
    let e = entries.into_iter().find(|e| named(e, name))?;
    let (offset, len) = (le64(e, NAME_LEN), le64(e, NAME_LEN + 8));
    let stop = offset.checked_add(len)?;
    if !offset.is_multiple_of(sector)
        || offset < floor
        || stop > STORE_END_LBA * sector
        || len > MOST
    {
        return None;
    }
    let mut data = sectors(disk, offset / sector, len.div_ceil(sector).max(1))?;
    data.truncate(len as usize);
    let digest = &e[NAME_LEN + 16..NAME_LEN + 32];
    (digest == [0u8; 16] || *digest == digest16(&data)).then_some(data)
}

/* The descriptor's name field holds `name` and NULs after it. */
fn named(e: &[u8], name: &str) -> bool {
    let (field, n) = (&e[..NAME_LEN], name.as_bytes());
    n.len() <= NAME_LEN && field[..n.len()] == *n && field[n.len()..].iter().all(|&b| b == 0)
}

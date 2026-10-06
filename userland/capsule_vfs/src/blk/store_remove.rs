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

//! Drops a named descriptor from the NONOSTR1 TOC. Payload extents stay
//! where they are, now free; the table loses one descriptor at a time by the
//! writes `store_drop` orders, header last, so a removal cut short leaves the
//! old table, the new one, or the dropped slot refused, never a name served
//! with another file's bytes.
use alloc::vec;

use nonos_disk_map::STORE_BASE_LBA;

use super::client::{capacity, read_blocks, read_span};
use super::error::BlkError;
use super::store_drop::drop_slot;
use super::store_header::{entry_count, ENTRY_LEN, HEADER_LEN, MAX_ENTRIES};
use super::store_toc::{decode, Window};
use super::store_write::write_sectors;
use super::wire::SECTOR_SIZE;

/// Every descriptor named `name`, one per pass and the table read afresh
/// each time, so a table that names it twice ends naming it neither time.
/// Slots, not entries: a damaged descriptor is not among the entries and is
/// kept as it stands, still refused and still reported, rather than dropped
/// here unseen.
pub fn remove(name: &str) -> Result<(), BlkError> {
    for _ in 0..=MAX_ENTRIES {
        let mut head = [0u8; SECTOR_SIZE];
        read_blocks(STORE_BASE_LBA, &mut head)?;
        let count = entry_count(&head)?;
        let mut toc = vec![0u8; sector_span(HEADER_LEN + ENTRY_LEN * count)];
        read_span(STORE_BASE_LBA, &mut toc)?;
        let end = capacity()?.checked_mul(SECTOR_SIZE as u64).ok_or(BlkError::BadLength)?;
        let window = Window { base: STORE_BASE_LBA * SECTOR_SIZE as u64, end };
        let entries = decode(&toc, count, window)?;
        let Some(slot) = entries.iter().find(|e| e.name == name).map(|e| e.slot) else {
            return Ok(());
        };
        for (sector, bytes) in drop_slot(&toc, count, slot)? {
            write_sectors(STORE_BASE_LBA + sector as u64, &bytes)?;
        }
    }
    Err(BlkError::BadContainer)
}

fn sector_span(bytes: usize) -> usize {
    bytes.div_ceil(SECTOR_SIZE) * SECTOR_SIZE
}

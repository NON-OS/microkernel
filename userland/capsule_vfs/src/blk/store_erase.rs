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

//! Erasing one table entry where it lies: its descriptor's bytes zeroed in
//! the sector or two that hold them, nothing else touched. Every byte it
//! changes becomes zero, which is all the kernel lets a boot that keeps
//! nothing write, and the next load reads the slot as empty.

use nonos_disk_map::STORE_BASE_LBA;

use super::error::BlkError;
use super::store_header::{ENTRY_LEN, HEADER_LEN};
use super::store_write::write_sectors;
use super::wire::SECTOR_SIZE;

/// `toc` (the table as read, from the store's first sector) with slot
/// `slot`'s descriptor zeroed, written back sector by sector. An entry
/// starts 32 bytes into a 128-byte step, so it may cross into a second
/// sector; both are written, and a cut between them leaves a descriptor
/// the next load refuses, never one that names another record's bytes.
pub fn erase_slot(toc: &[u8], slot: usize) -> Result<(), BlkError> {
    let at = ENTRY_LEN
        .checked_mul(slot)
        .and_then(|o| o.checked_add(HEADER_LEN))
        .ok_or(BlkError::BadContainer)?;
    let end = at.checked_add(ENTRY_LEN).ok_or(BlkError::BadContainer)?;
    if end > toc.len() {
        return Err(BlkError::BadContainer);
    }
    let mut region = toc.to_vec();
    region[at..end].fill(0);
    let first = at / SECTOR_SIZE;
    let last = (end - 1) / SECTOR_SIZE;
    let span = &region[first * SECTOR_SIZE..(last + 1) * SECTOR_SIZE];
    write_sectors(STORE_BASE_LBA + first as u64, span)
}

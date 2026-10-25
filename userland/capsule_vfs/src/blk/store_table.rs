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
//! The store's table as the disk holds it now, read once per write.

use alloc::vec;

use nonos_disk_map::{STORE_BASE_LBA, TOC_SPAN};

use super::client::{capacity, read_blocks, read_span};
use super::error::BlkError;
use super::store_header::{entry_count, ENTRY_LEN, HEADER_LEN};
use super::store_toc::{decode, Window};
use super::store_write::sector_span;
use super::wire::SECTOR_SIZE;

/// The table as the disk holds it now, with the bounds a write is kept to.
pub(super) struct Table {
    pub(super) toc: alloc::vec::Vec<u8>,
    pub(super) count: usize,
    pub(super) base: u64,
    pub(super) floor: u64,
    pub(super) capacity_bytes: u64,
    pub(super) entries: super::store_toc::Toc,
}

impl Table {
    pub(super) fn read() -> Result<Self, BlkError> {
        let mut head = [0u8; SECTOR_SIZE];
        read_blocks(STORE_BASE_LBA, &mut head)?;
        let count = entry_count(&head)?;
        let mut toc = vec![0u8; sector_span(HEADER_LEN + ENTRY_LEN * count)];
        read_span(STORE_BASE_LBA, &mut toc)?;
        let capacity_bytes = capacity()?.checked_mul(SECTOR_SIZE as u64).ok_or(BlkError::BadLength)?;
        let base = STORE_BASE_LBA * SECTOR_SIZE as u64;
        /*
         * Payloads go past a full table, so nothing this writes is ever in the
         * way of the table growing again.
         */
        let floor = base + TOC_SPAN as u64;
        let entries = decode(&toc, count, Window { base, end: capacity_bytes })?;
        Ok(Self { toc, count, base, floor, capacity_bytes, entries })
    }
}

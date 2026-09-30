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

//! The package store's container, from `STORE_BASE_LBA` on.
//!
//! ```text
//!   header, 32 bytes   0..8 magic, 8..12 version, 12..16 entry count (LE)
//!   entry, 128 bytes   0..96 path, NUL padded; 96..104 payload offset in
//!                      bytes from the start of the disk; 104..112 length;
//!                      112..128 digest16 of the payload, zero for none
//! ```
//!
//! Entries follow the header. Payloads start on sector boundaries.

use crate::places::SECTOR_SIZE;

pub const STORE_MAGIC: [u8; 8] = *b"NONOSTR1";
pub const STORE_VERSION: u32 = 1;
pub const HEADER_LEN: usize = 32;
pub const ENTRY_LEN: usize = 128;
pub const NAME_LEN: usize = 96;

/// Entries a table may hold. The table is decoded into a heap vector and
/// every payload byte is still bounded by `MAX_TOTAL_BYTES`, so this only
/// sizes the table: 128 entries are 16 KiB.
pub const MAX_ENTRIES: usize = 128;

/// Payload bytes vfs loads at most, summed over the entries, so a hostile
/// table cannot exhaust the heap vfs loads the store into.
pub const MAX_TOTAL_BYTES: u64 = 16 * 1024 * 1024;

/// The header and a full table, in whole sectors. vfs appends its first
/// payload past this, so a table that grows to `MAX_ENTRIES` never runs
/// into a payload.
pub const TOC_SPAN: usize =
    (HEADER_LEN + ENTRY_LEN * MAX_ENTRIES).div_ceil(SECTOR_SIZE) * SECTOR_SIZE;

/// The one test for a path in the table: a name that writes is a name that
/// decodes, since one entry that fails to decode fails the whole table.
pub fn valid_name(name: &str) -> bool {
    !name.is_empty() && name.is_ascii() && name.len() <= NAME_LEN && !name.as_bytes().contains(&0)
}

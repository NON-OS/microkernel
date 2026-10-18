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

//! What partition entry array a header may name and how much of it is read
//! (UEFI 2.10, 5.3.2, as recalled, unverified): entries of 128 × 2^n bytes,
//! at least one, wholly inside the room which.rs gives the header, so no
//! partition's I/O can reach it.

/// The fields read lie in an entry's first 128 bytes; a larger entry is
/// legal and read for those alone, so a table written with them still
/// places its partitions.
const ENTRY_MIN: usize = 128;
/// The most array read: 1024 entries of 128 bytes. It bounds what one
/// table makes the kernel allocate and read, whatever the header claims.
const ARRAY_MAX: u64 = 128 * 1024;

/// Sectors of 512 bytes that `entries` of `size` bytes fill.
pub(crate) fn array_sectors(entries: u32, size: u32) -> u64 {
    (entries as u64 * size as u64).div_ceil(512)
}

/// Whether `entries` of `size` bytes are an array to read: entries of
/// 128 x 2^n bytes, at least one, at most `ARRAY_MAX` in all.
pub(crate) fn shaped(entries: u32, size: u32) -> bool {
    let bytes = entries as u64 * size as u64;
    size as usize >= ENTRY_MIN && size.is_power_of_two() && entries >= 1 && bytes <= ARRAY_MAX
}

/// Whether that array at `lba` lies wholly in sectors `lo` up to `hi`.
pub(crate) fn within(lba: u64, entries: u32, size: u32, lo: u64, hi: u64) -> bool {
    lba >= lo && lba.saturating_add(array_sectors(entries, size)) <= hi
}

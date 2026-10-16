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

//! Where a header's partition entry array may lie and how much of it is
//! read (UEFI 2.10, 5.3.2, as recalled, unverified): entries of 128 × 2^n
//! bytes, at least one, the array after the header at LBA 1 and wholly
//! before FirstUsableLBA, so no partition's I/O can reach it.

/// The fields read lie in an entry's first 128 bytes; a larger entry is
/// legal and read for those alone, so a table written with them still
/// places its partitions.
pub(crate) const ENTRY_MIN: usize = 128;
/// The most array read: 1024 entries of 128 bytes. It bounds what one
/// table makes the kernel allocate and read, whatever the header claims.
const ARRAY_MAX: u64 = 128 * 1024;

/// Sectors of 512 bytes that `entries` of `size` bytes fill.
pub(crate) fn array_sectors(entries: u32, size: u32) -> u64 {
    (entries as u64 * size as u64).div_ceil(512)
}

/// Whether an array of `entries` of `size` bytes at `lba` is one to read,
/// below a usable range starting at `first_usable`.
pub(crate) fn array_fits(lba: u64, entries: u32, size: u32, first_usable: u64) -> bool {
    let size_ok = size as usize >= ENTRY_MIN && size.is_power_of_two();
    let bytes = entries as u64 * size as u64;
    let end = lba.saturating_add(array_sectors(entries, size));
    size_ok && entries >= 1 && bytes <= ARRAY_MAX && lba >= 2 && end <= first_usable
}

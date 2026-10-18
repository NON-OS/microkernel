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

//! Where the kernel reads the loader's memory map. Pure, so the host proofs
//! hold it.
//!
//! The map was read at its physical address, through the low identity
//! window the loader leaves, with `entry_count` taken as given. The kernel
//! tears that window down after its first page-table setup, so a later
//! reader would have faulted, and a count from a broken loader walked off
//! the end of the buffer. It is now read through the direct map, and a
//! window the direct map does not hold whole is no map at all.

/// More descriptors than any firmware reports; the loader sizes its buffer
/// from the real map, a few thousand entries on a large server.
pub const MAX_MAP_ENTRIES: u32 = 1 << 16;

/// The direct-map address of a map of `count` entries of `entry_size`
/// bytes at physical `ptr`, and its entry count, given the direct map at
/// `base` covering `size` bytes of physical memory. `None` for an empty
/// map, a count past `MAX_MAP_ENTRIES`, a misaligned pointer, or a map that
/// runs past the direct map.
pub const fn map_window(
    ptr: u64,
    entry_size: u32,
    count: u32,
    base: u64,
    size: u64,
) -> Option<(u64, usize)> {
    if ptr == 0 || count == 0 || count > MAX_MAP_ENTRIES || !ptr.is_multiple_of(8) {
        return None;
    }
    let bytes = entry_size as u64 * count as u64;
    match ptr.checked_add(bytes) {
        Some(end) if end <= size => Some((base + ptr, count as usize)),
        _ => None,
    }
}

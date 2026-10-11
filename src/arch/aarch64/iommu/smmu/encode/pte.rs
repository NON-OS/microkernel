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

//! Stage 1 translation table entries, VMSAv8-64 with a 4 KiB granule.

pub const ENTRIES: usize = 512;
/// Levels below the one TTB0 points at: 39-bit input addresses start at
/// level 1, so a walk is level 1, 2 and the level 3 leaves.
pub const LEVELS: usize = 3;
const OA_MASK: u64 = ((1 << 48) - 1) & !0xfff;

const VALID_TABLE: u64 = 0b11;
const VALID_PAGE: u64 = 0b11;
/// AP[1]: the unit tags device accesses unprivileged.
const AP_UNPRIV: u64 = 1 << 6;
/// AP[2]: read-only.
const AP_RDONLY: u64 = 1 << 7;
const SH_INNER: u64 = 0b11 << 8;
const AF: u64 = 1 << 10;
/// No device fetches instructions.
const XN: u64 = 1 << 54 | 1 << 53;

pub const fn table(next_phys: u64) -> u64 {
    VALID_TABLE | (next_phys & OA_MASK)
}

/// A leaf for one 4 KiB page, attribute index 0. Stage 1 permissions are
/// read-only or read-write, so a write-only grant maps read-write; `None`
/// for one that allows neither, which has no encoding but "not mapped".
pub const fn page(phys: u64, read: bool, write: bool) -> Option<u64> {
    if !read && !write {
        return None;
    }
    let ap = if write { AP_UNPRIV } else { AP_UNPRIV | AP_RDONLY };
    Some(VALID_PAGE | (phys & OA_MASK) | ap | SH_INNER | AF | XN)
}

pub const fn is_valid(entry: u64) -> bool {
    entry & 1 != 0
}

pub const fn address(entry: u64) -> u64 {
    entry & OA_MASK
}

/// The index `iova` takes in a table at `level` (1, 2 or 3).
pub const fn level_index(iova: u64, level: usize) -> usize {
    ((iova >> (12 + 9 * (3 - level))) & 0x1ff) as usize
}

/// Every IOVA must fit the 39 bits T0SZ leaves.
pub const fn in_range(iova: u64, size: u64) -> bool {
    match iova.checked_add(size) {
        Some(end) => end <= 1 << 39,
        None => false,
    }
}

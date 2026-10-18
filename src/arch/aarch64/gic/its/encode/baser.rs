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

//! `GITS_BASER<n>`, the tables the ITS keeps in memory.

use super::attrs::attrs;

pub const TYPE_DEVICES: u8 = 1;
pub const TYPE_COLLECTIONS: u8 = 4;

pub const PAGE_4K: u64 = 0;
pub const PAGE_16K: u64 = 1 << 8;
pub const PAGE_64K: u64 = 2 << 8;
const PAGE_MASK: u64 = 3 << 8;
/// `Size` is the page count less one, in eight bits.
pub const MAX_PAGES: u64 = 256;

const VALID: u64 = 1 << 63;
/// Type and Entry_Size: read-only, kept as read.
const READ_ONLY: u64 = 7 << 56 | 0x1f << 48;

pub const fn table_type(baser: u64) -> u8 {
    ((baser >> 56) & 7) as u8
}

pub const fn entry_bytes(baser: u64) -> u64 {
    ((baser >> 48) & 0x1f) + 1
}

pub const fn page_bytes(page: u64) -> u64 {
    match page & PAGE_MASK {
        PAGE_4K => 0x1000,
        PAGE_16K => 0x4000,
        _ => 0x1_0000,
    }
}

pub const fn page_of(baser: u64) -> u64 {
    baser & PAGE_MASK
}

/// A valid flat table of `pages` (1 to `MAX_PAGES`) pages at `phys`. A 64 KiB
/// page keeps address bits 51:48 in 15:12.
pub const fn table(read: u64, phys: u64, page: u64, pages: u64, coherent: bool) -> u64 {
    let pa = if page == PAGE_64K {
        (phys & 0x0000_ffff_ffff_0000) | ((phys >> 48) & 0xf) << 12
    } else {
        phys & 0x0000_ffff_ffff_f000
    };
    VALID | (read & READ_ONLY) | attrs(coherent) | pa | page | (pages - 1)
}

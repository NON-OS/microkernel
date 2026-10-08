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

//! AMD IOMMU page table entries (spec 48882, section 2.2.3), as Linux builds
//! them in amd/io_pgtable.c: PR in bit 0, the next level in bits 11:9, the
//! address in 51:12, FC in 60, IR in 61 and IW in 62. A directory names the
//! level of the table below it; a 4 KiB leaf at level 1 names level 0. Pure.

/// Four levels reach 48 bits, the width a capsule's IOVAs use.
pub const LEVELS: u8 = 4;
pub const ENTRIES: usize = 512;

const PRESENT: u64 = 1 << 0;
const FORCE_COHERENT: u64 = 1 << 60;
const READ: u64 = 1 << 61;
const WRITE: u64 = 1 << 62;
const ADDR_MASK: u64 = 0x000F_FFFF_FFFF_F000;

/// A directory entry at `level` (2 or more) pointing at the table below.
/// IR and IW are set so the leaf alone decides, as Linux does.
pub const fn directory(table_phys: u64, level: u8) -> u64 {
    PRESENT | (((level as u64 - 1) & 0x7) << 9) | (table_phys & ADDR_MASK) | READ | WRITE
}

/// A 4 KiB leaf. FC makes the device's access snoop CPU caches whatever the
/// request asked, so a driver's write-back buffers stay coherent.
pub const fn leaf(phys: u64, read: bool, write: bool) -> u64 {
    let r = if read { READ } else { 0 };
    let w = if write { WRITE } else { 0 };
    PRESENT | FORCE_COHERENT | (phys & ADDR_MASK) | r | w
}

pub const fn is_present(entry: u64) -> bool {
    entry & PRESENT != 0
}

pub const fn next_level(entry: u64) -> u8 {
    ((entry >> 9) & 0x7) as u8
}

pub const fn address(entry: u64) -> u64 {
    entry & ADDR_MASK
}

pub const fn writable(entry: u64) -> bool {
    entry & WRITE != 0
}

/// The slot `iova` uses in a table at `level`, nine bits per level above
/// the 4 KiB page offset.
pub const fn index(iova: u64, level: u8) -> usize {
    ((iova >> (12 + 9 * (level as u32 - 1))) & 0x1FF) as usize
}

/// Bytes one table of `levels` levels reaches.
pub const fn reach(levels: u8) -> u64 {
    1u64 << (12 + 9 * levels as u32)
}

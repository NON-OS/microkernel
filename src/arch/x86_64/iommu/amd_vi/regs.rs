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

//! AMD IOMMU MMIO registers (AMD I/O Virtualization Technology (IOMMU)
//! Specification 48882, rev 3.x, section 3.4) and the values the kernel
//! writes into them. Pure.

/// The registers the kernel uses end at 0x2028; Linux maps the same 16 KiB
/// (amd_iommu_types.h, MMIO_REGION_LENGTH).
pub const WINDOW: usize = 0x4000;

pub const DEV_TABLE_BASE: usize = 0x0000;
pub const CMD_BUF_BASE: usize = 0x0008;
pub const EVENT_LOG_BASE: usize = 0x0010;
pub const EXT_FEATURE: usize = 0x0030;
pub const CMD_HEAD: usize = 0x2000;
pub const CMD_TAIL: usize = 0x2008;
pub const EVENT_HEAD: usize = 0x2010;
pub const EVENT_TAIL: usize = 0x2018;
pub const STATUS: usize = 0x2020;

/// Status bits, write one to clear.
pub const STATUS_EVENT_OVERFLOW: u64 = 1 << 0;

/// Extended Feature Register IASup, bit 6: INVALIDATE_IOMMU_ALL is accepted.
pub const fn invalidate_all_supported(efr: u64) -> bool {
    efr & (1 << 6) != 0
}

/// One device table covering every requester id: 65536 entries of 32 bytes,
/// 512 pages, so no device id can index past it (Linux allocates the same).
pub const DEV_TABLE_PAGES: usize = 512;

/// Device Table Base: the base, and in bits 8:0 the size in pages less one.
pub const fn dev_table_base(phys: u64, pages: usize) -> u64 {
    (phys & 0x000F_FFFF_FFFF_F000) | ((pages as u64 - 1) & 0x1FF)
}

/// One page of 16-byte entries for the command buffer and the event log.
pub const RING_ENTRIES: u16 = 256;
const RING_LOG2: u64 = 8;

/// Command Buffer or Event Log Base: the base, and in bits 59:56 the log2 of
/// the entry count (ComLen, EventLen).
pub const fn ring_base(phys: u64) -> u64 {
    (phys & 0x000F_FFFF_FFFF_F000) | (RING_LOG2 << 56)
}

/// Head and tail registers hold a byte offset in bits 18:4.
pub const fn ring_offset(index: u16) -> u64 {
    (index as u64) << 4
}

pub const fn ring_index(register: u64) -> u16 {
    (((register >> 4) & 0x7FFF) as u16) % RING_ENTRIES
}

pub const fn ring_next(index: u16) -> u16 {
    (index + 1) % RING_ENTRIES
}

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

//! Event log entries (AMD IOMMU spec 48882, section 2.5), four dwords, read
//! the way Linux reads them in amd/iommu.c (iommu_print_event). Pure.

pub type Event = [u32; 4];

pub const IO_PAGE_FAULT: u8 = 0x2;

/// The four dwords of an entry read as two little-endian quadwords.
pub const fn from_words(low: u64, high: u64) -> Event {
    [low as u32, (low >> 32) as u32, high as u32, (high >> 32) as u32]
}

/// The event code, bits 31:28 of the second dword. Zero means the unit has
/// not finished writing the entry yet.
pub const fn code(event: Event) -> u8 {
    (event[1] >> 28) as u8
}

pub const fn device_id(event: Event) -> u16 {
    event[0] as u16
}

/// The domain or PASID the fault was taken under (second dword, 15:0).
pub const fn domain(event: Event) -> u16 {
    event[1] as u16
}

/// Fault flags, bits 27:16 of the second dword.
pub const fn flags(event: Event) -> u16 {
    ((event[1] >> 16) & 0xFFF) as u16
}

/// RW in the flags: the faulting request was a write.
pub const fn is_write(event: Event) -> bool {
    flags(event) & 0x020 != 0
}

pub const fn address(event: Event) -> u64 {
    ((event[3] as u64) << 32) | event[2] as u64
}

pub const fn code_text(code: u8) -> &'static [u8] {
    match code {
        0x1 => b"illegal device table entry",
        0x2 => b"page fault",
        0x3 => b"device table hardware error",
        0x4 => b"page table hardware error",
        0x5 => b"illegal command",
        0x6 => b"command hardware error",
        0x7 => b"IOTLB invalidation timed out",
        0x8 => b"invalid device request",
        0x9 => b"invalid PPR request",
        0xA => b"event counter zero",
        _ => b"event code not listed",
    }
}

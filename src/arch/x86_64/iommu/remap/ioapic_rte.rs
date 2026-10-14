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

//! An IO-APIC redirection entry in remappable format (VT-d 3.4, 5.1.5.1):
//! it names a remapping entry instead of a CPU, laid out as Linux's
//! IR_IO_APIC_route_entry. Pure; kernel_proofs::iommu_remap::rte_tests.

/// Interrupt Format, bit 48 of the entry (bit 16 of the high dword).
const FORMAT_REMAPPABLE: u32 = 1 << 16;
/// Handle bit 15, in bit 11 of the low dword.
const HANDLE_HIGH_BIT: u32 = 1 << 11;
const ACTIVE_LOW: u32 = 1 << 13;
const LEVEL: u32 = 1 << 15;
const MASKED: u32 = 1 << 16;

/// The low and high dwords naming entry `handle`. The vector is the entry's
/// own: the IO-APIC matches a level line's EOI by it.
pub const fn remappable(handle: u16, vector: u8, level: bool, active_low: bool) -> (u32, u32) {
    let mut low = vector as u32 | MASKED;
    if handle & 0x8000 != 0 {
        low |= HANDLE_HIGH_BIT;
    }
    if active_low {
        low |= ACTIVE_LOW;
    }
    if level {
        low |= LEVEL;
    }
    (low, FORMAT_REMAPPABLE | ((handle as u32 & 0x7FFF) << 17))
}

/// The entry an IO-APIC entry names, or None in compatibility format.
pub const fn handle_of(low: u32, high: u32) -> Option<u16> {
    if high & FORMAT_REMAPPABLE == 0 {
        return None;
    }
    let top = if low & HANDLE_HIGH_BIT != 0 { 0x8000 } else { 0 };
    Some(top | (high >> 17) as u16)
}

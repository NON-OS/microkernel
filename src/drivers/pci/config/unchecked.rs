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

//! The scan path's accesses, which skip validation because the caller already
//! bounded the offset to a byte.

use super::transport;

/// An absent function reads as all-ones, which is how the bus reports one and
/// what the scan is looking for, so an unreachable config space collapses
/// into the same answer rather than a new error case.
///
/// The offset is rounded down to its dword, as the legacy address register
/// does in hardware: callers pass a register's own offset (CFG_STATUS, 0x06)
/// and shift. ECAM is memory mapped as Device memory, where a 32-bit load at
/// 0x06 is an alignment fault on aarch64, so the rounding is done here for
/// both transports.
#[inline]
pub fn read32_unchecked(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    transport::read32(bus, device, function, dword(offset)).unwrap_or(!0)
}

/// A write that cannot reach the function is dropped, as a write to an absent
/// function is on the bus.
#[inline]
pub fn write32_unchecked(bus: u8, device: u8, function: u8, offset: u8, value: u32) {
    let _ = transport::write32(bus, device, function, dword(offset), value);
}

#[inline]
const fn dword(offset: u8) -> u16 {
    (offset & 0xFC) as u16
}

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

//! One PRDT entry (JESD223, 6.1.2): the buffer's address and its byte count
//! less one, which must be a whole number of dwords.

use super::layout::PRDT_ENTRY;

/// The entry for `len` bytes at `bus`; None for an empty, unaligned or
/// over-long (above 256 KiB) buffer, which the host would refuse.
pub fn entry(bus: u64, len: usize) -> Option<[u8; PRDT_ENTRY]> {
    if len == 0 || !len.is_multiple_of(4) || len > 1 << 18 || !bus.is_multiple_of(4) {
        return None;
    }
    let mut out = [0u8; PRDT_ENTRY];
    out[0..8].copy_from_slice(&bus.to_le_bytes());
    out[12..16].copy_from_slice(&((len - 1) as u32).to_le_bytes());
    Some(out)
}

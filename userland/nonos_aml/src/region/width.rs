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

//! How wide a field's accesses are (section 19.6.48, AccessType).

use super::Space;

/// The access width in bytes for a field of `bit_len` bits at `bit_offset`,
/// in a region of `room` bytes. ByteAcc to QWordAcc fix it. AnyAcc leaves
/// it to the operating system: this takes the narrowest naturally aligned
/// width, up to eight bytes, that holds the whole field in one access
/// inside the region, so a register declared AnyAcc is still read in one
/// piece and no byte past the region is touched; a field no such width
/// holds is read by bytes. An embedded controller or CMOS field is always
/// read by bytes.
pub fn access_width(
    access_type: u8,
    bit_offset: u32,
    bit_len: u32,
    space: Space,
    room: u64,
) -> u32 {
    if space.is_byte_wide() {
        return 1;
    }
    match access_type & 0x0F {
        1 => 1,
        2 => 2,
        3 => 4,
        4 => 8,
        _ => {
            let (start, end) = (bit_offset as u64, bit_offset as u64 + bit_len as u64);
            [1u64, 2, 4, 8]
                .into_iter()
                .find(|w| {
                    let unit = start / (w * 8);
                    unit == (end.max(start + 1) - 1) / (w * 8) && (unit + 1) * w <= room
                })
                .unwrap_or(1) as u32
        }
    }
}

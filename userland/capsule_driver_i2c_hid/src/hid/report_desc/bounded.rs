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

//! A field as the descriptor declares it, kept only when it can be read.
//! Report Size and the offsets it adds up to are the device's numbers:
//! one of 2^32 bits, or one past any report, would make every sum over
//! fields overflow. A field this driver reads is a value of 1 to 32 bits
//! inside the longest report a 16-bit length can describe; any other is
//! left unrecorded, as if the descriptor had not declared it.

use super::layout::Field;

/// The bits in the longest report: 65535 bytes, the most a 16-bit length
/// counts (HID over I2C's input length, HID over SPI's content length).
pub const REPORT_BITS_MAX: u32 = 65_535 * 8;

impl Field {
    pub fn bounded(bit_offset: u32, bit_size: u32, logical_max: i32) -> Option<Field> {
        let end = bit_offset.checked_add(bit_size)?;
        let sound = (1..=32).contains(&bit_size) && end <= REPORT_BITS_MAX;
        sound.then_some(Field { bit_offset, bit_size, logical_max })
    }
}

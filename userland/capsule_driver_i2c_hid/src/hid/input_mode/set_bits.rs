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

//! One bit field of a report body, written in place.

// Write a value into a bit field of a report body. Returns whether any bit
// changed. Out-of-range offsets leave the body untouched.
pub(super) fn set_bits(body: &mut [u8], bit_offset: u32, bit_size: u32, value: u32) -> bool {
    if bit_size == 0 || bit_size > 32 {
        return false;
    }
    let end = bit_offset as usize + bit_size as usize;
    if end > body.len() * 8 {
        return false;
    }
    let mut changed = false;
    for i in 0..bit_size {
        let bit = ((value >> i) & 1) as u8;
        let at = bit_offset as usize + i as usize;
        let byte = at / 8;
        let mask = 1u8 << (at % 8);
        let current = (body[byte] & mask != 0) as u8;
        if current != bit {
            body[byte] = (body[byte] & !mask) | (bit << (at % 8));
            changed = true;
        }
    }
    changed
}

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

//! Bits in a little-endian byte string: a field's value, a buffer field's
//! buffer.

/// `count` bits (at most 64) starting at bit `at`; bits past the end read
/// as zero.
pub fn get_bits(bytes: &[u8], at: u64, count: u32) -> u64 {
    let mut v = 0u64;
    for i in 0..count.min(64) as u64 {
        let bit = at + i;
        let byte = bytes.get((bit / 8) as usize).copied().unwrap_or(0);
        v |= (((byte >> (bit % 8)) & 1) as u64) << i;
    }
    v
}

/// Writes the low `count` bits (at most 64) of `v` from bit `at`; bits past
/// the end are dropped.
pub fn set_bits(bytes: &mut [u8], at: u64, count: u32, v: u64) {
    for i in 0..count.min(64) as u64 {
        let bit = at + i;
        if let Some(byte) = bytes.get_mut((bit / 8) as usize) {
            let mask = 1u8 << (bit % 8);
            if (v >> i) & 1 == 1 {
                *byte |= mask;
            } else {
                *byte &= !mask;
            }
        }
    }
}

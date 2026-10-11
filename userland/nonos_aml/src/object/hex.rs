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

//! Hexadecimal and decimal digits, both ways.

use alloc::vec::Vec;

pub fn hex_value(b: u8) -> Option<u64> {
    (b as char).to_digit(16).map(u64::from)
}

/// `v` as `digits` uppercase hexadecimal characters, leading zeros kept.
pub fn hex_fixed(v: u64, digits: usize, out: &mut Vec<u8>) {
    for i in (0..digits).rev() {
        out.push(b"0123456789ABCDEF"[((v >> (4 * i)) & 0xF) as usize]);
    }
}

/// `v` as uppercase hexadecimal without leading zeros ("0" for zero).
pub fn hex_trimmed(v: u64, out: &mut Vec<u8>) {
    let digits = (64 - v.leading_zeros() as usize).div_ceil(4);
    hex_fixed(v, digits.max(1), out);
}

pub fn decimal(mut v: u64, out: &mut Vec<u8>) {
    let start = out.len();
    loop {
        out.push(b'0' + (v % 10) as u8);
        v /= 10;
        if v == 0 {
            break;
        }
    }
    out[start..].reverse();
}

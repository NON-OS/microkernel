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

//! Writing the hex and decimal text of a request.

use alloc::vec::Vec;

use super::hex_digit::hex_digit;

pub fn append_dec_u64(out: &mut Vec<u8>, mut n: u64) {
    let mut buf = [0u8; 20];
    let mut i = buf.len();
    if n == 0 {
        out.push(b'0');
        return;
    }
    while n != 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    out.extend_from_slice(&buf[i..]);
}

pub fn append_hex20(out: &mut Vec<u8>, bytes: &[u8; 20]) {
    out.extend_from_slice(b"0x");
    for b in bytes {
        out.push(hex_digit(b >> 4));
        out.push(hex_digit(*b));
    }
}

pub fn append_hex_bytes(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(b"0x");
    for b in bytes {
        out.push(hex_digit(b >> 4));
        out.push(hex_digit(*b & 15));
    }
}

/// A JSON-RPC quantity: "0x" and the shortest hex, "0x0" for zero.
pub fn append_quantity(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(b"0x");
    let digits = if v == 0 { 1 } else { (64 - v.leading_zeros() as usize).div_ceil(4) };
    for i in (0..digits).rev() {
        out.push(hex_digit(((v >> (4 * i)) & 15) as u8));
    }
}

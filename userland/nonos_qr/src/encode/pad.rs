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

//! The data codewords of a byte-mode message: mode, count, data, terminator,
//! bit padding to a byte, then the 0xEC 0x11 pad bytes up to `cap`.

use alloc::vec::Vec;

use super::bitbuf::BitBuf;

/// The `cap` data codewords carrying `data` in byte mode at `version`.
pub(super) fn padded_data(data: &[u8], version: u8, cap: usize) -> Vec<u8> {
    let mut bits = BitBuf::new();
    bits.push(0b0100, 4);
    let count_bits = if version <= 9 { 8 } else { 16 };
    bits.push(data.len() as u32, count_bits);
    for &b in data {
        bits.push(b as u32, 8);
    }
    let cap_bits = cap * 8;
    let term = core::cmp::min(4, cap_bits.saturating_sub(bits.len()));
    bits.push(0, term as u32);
    while !bits.len().is_multiple_of(8) {
        bits.push(0, 1);
    }
    let mut bytes = bits.into_bytes();
    let pads = [0xECu8, 0x11u8];
    let mut i = 0;
    while bytes.len() < cap {
        bytes.push(pads[i % 2]);
        i += 1;
    }
    bytes
}

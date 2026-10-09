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

//! Numbers on a line: decimal, and hex with or without the `0x`.

use super::buffer::Line;

impl Line {
    /// Append `v` in decimal.
    pub fn dec(&mut self, v: u64) -> &mut Self {
        let mut digits = [0u8; 20];
        let mut n = 0usize;
        let mut v = v;
        loop {
            digits[n] = b'0' + (v % 10) as u8;
            n += 1;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        digits[..n].reverse();
        self.text(&digits[..n])
    }

    /// Append `v` as `0x` and exactly `width` hex digits (at most 16), the
    /// way register values and PCI ids are read.
    pub fn hex(&mut self, v: u64, width: usize) -> &mut Self {
        self.text(b"0x").hex_digits(v, width)
    }

    /// `hex` without the `0x`, for a `vendor:device` pair.
    pub fn hex_digits(&mut self, v: u64, width: usize) -> &mut Self {
        let width = width.clamp(1, 16);
        let mut digits = [0u8; 16];
        for (i, d) in digits[..width].iter_mut().enumerate() {
            let nibble = (v >> (4 * (width - 1 - i))) & 0xf;
            *d = b"0123456789abcdef"[nibble as usize];
        }
        self.text(&digits[..width])
    }
}

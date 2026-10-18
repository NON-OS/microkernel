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

//! Appending ASCII text, hex and decimal numbers to a line.

use super::Line;

impl Line {
    /// Append printable ASCII from `b`, a `?` for anything else, stopping
    /// at the first NUL.
    pub fn ascii(&mut self, b: &[u8]) -> &mut Self {
        for &c in b {
            if c == 0 {
                break;
            }
            let p = if (0x20..0x7f).contains(&c) { c } else { b'?' };
            self.s(&[p]);
        }
        self
    }

    /// Append `v` as `0x` and its hex digits, without leading zeros.
    pub fn hex(&mut self, v: u64) -> &mut Self {
        self.s(b"0x");
        let mut digits = [0u8; 16];
        let mut i = 16;
        let mut x = v;
        loop {
            i -= 1;
            let d = (x & 0xf) as u8;
            digits[i] = if d < 10 { b'0' + d } else { b'a' + d - 10 };
            x >>= 4;
            if x == 0 {
                break;
            }
        }
        let tail = digits;
        self.s(&tail[i..])
    }

    /// Append `v` in decimal.
    pub fn dec(&mut self, v: u64) -> &mut Self {
        let mut digits = [0u8; 20];
        let mut i = 20;
        let mut x = v;
        loop {
            i -= 1;
            digits[i] = b'0' + (x % 10) as u8;
            x /= 10;
            if x == 0 {
                break;
            }
        }
        let tail = digits;
        self.s(&tail[i..])
    }
}

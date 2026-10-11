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

//! Numbers in a console line: hex for IDs and addresses, decimal for speeds.
//! Each digit goes through the same bounded byte path as text, so a number
//! can never run a line past its room.

use super::line::Line;

const HEX: &[u8; 16] = b"0123456789abcdef";

impl Line {
    /// `digits` lowercase hex digits of `v`, most significant first.
    pub fn hex(&mut self, v: u32, digits: u32) -> &mut Self {
        for i in (0..digits.min(8)).rev() {
            self.byte(HEX[((v >> (i * 4)) & 0xF) as usize]);
        }
        self
    }

    pub fn dec(&mut self, v: u32) -> &mut Self {
        let mut digits = [0u8; 10];
        let (mut n, mut v) = (0, v);
        loop {
            digits[n] = b'0' + (v % 10) as u8;
            n += 1;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        for &d in digits[..n].iter().rev() {
            self.byte(d);
        }
        self
    }
}

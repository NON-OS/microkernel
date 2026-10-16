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

//! A line written into a fixed buffer, cut rather than overflowed.

/// Writes into a fixed buffer and stops at its end: a long line is cut, never
/// an overflow.
pub(super) struct Writer<'a> {
    pub(super) out: &'a mut [u8; 48],
    pub(super) len: usize,
}

impl<'a> Writer<'a> {
    pub(super) fn text(&mut self, b: &[u8]) {
        for &c in b {
            if self.len == self.out.len() {
                return;
            }
            self.out[self.len] = c;
            self.len += 1;
        }
    }
    pub(super) fn num(&mut self, mut n: u64) {
        let mut digits = [0u8; 20];
        let mut i = digits.len();
        loop {
            i -= 1;
            digits[i] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        let tmp = digits;
        self.text(&tmp[i..]);
    }
    pub(super) fn done(self) -> &'a [u8] {
        let len = self.len;
        &self.out[..len]
    }
}

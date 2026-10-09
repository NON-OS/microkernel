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

//! One console line, built without an allocator or a formatter, tagged
//! "rtsx: " so `log rtsx` shows every step. Pure.

/// The kernel takes at most 256 bytes per debug call.
pub const LINE_BYTES: usize = 160;

pub struct Line {
    buf: [u8; LINE_BYTES],
    len: usize,
}

impl Line {
    pub fn start() -> Self {
        let mut line = Self { buf: [0; LINE_BYTES], len: 0 };
        line.text(b"rtsx: ");
        line
    }

    /// Append bytes, cut at the room left for the newline.
    pub fn text(&mut self, s: &[u8]) -> &mut Self {
        for &b in s {
            if self.len == LINE_BYTES - 1 {
                break;
            }
            self.buf[self.len] = b;
            self.len += 1;
        }
        self
    }

    pub fn dec(&mut self, v: u64) -> &mut Self {
        let mut digits = [0u8; 20];
        let (mut n, mut v) = (0usize, v);
        while n == 0 || v != 0 {
            digits[19 - n] = b'0' + (v % 10) as u8;
            v /= 10;
            n += 1;
        }
        self.text(&digits[20 - n..])
    }

    /// `width` hex digits (at most 16), no prefix, as ids and registers read.
    pub fn hex(&mut self, v: u64, width: usize) -> &mut Self {
        let width = width.clamp(1, 16);
        let mut digits = [0u8; 16];
        for (i, d) in digits[..width].iter_mut().enumerate() {
            *d = b"0123456789abcdef"[((v >> (4 * (width - 1 - i))) & 0xf) as usize];
        }
        self.text(&digits[..width])
    }

    /// The line as the console gets it, newline included.
    pub fn finish(&mut self) -> &[u8] {
        self.buf[self.len] = b'\n';
        &self.buf[..self.len + 1]
    }
}

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

use nonos_libc::mk_debug;

const ROOM: usize = 160;

/// One console line, built without allocating and cut to fit. It always
/// ends in a newline, so the next line on the serial console starts clean.
pub struct Line {
    buf: [u8; ROOM],
    len: usize,
}

impl Line {
    pub fn new(text: &str) -> Self {
        Self { buf: [0; ROOM], len: 0 }.text(text)
    }

    pub fn text(mut self, text: &str) -> Self {
        text.bytes().for_each(|b| self.push(b));
        self
    }

    /// `v` in lower-case hex after "0x", without leading zeros.
    pub fn hex(mut self, v: u32) -> Self {
        self = self.text("0x");
        let digits = (32 - v.leading_zeros()).div_ceil(4).max(1);
        for i in (0..digits).rev() {
            self.push(b"0123456789abcdef"[((v >> (i * 4)) & 0xF) as usize]);
        }
        self
    }

    /// `v` in decimal, zero-padded to `width` digits.
    pub fn dec(mut self, v: u32, width: usize) -> Self {
        let mut tmp = [b'0'; 10];
        let (mut n, mut used) = (v, 0);
        while n > 0 || used == 0 {
            tmp[9 - used] = b'0' + (n % 10) as u8;
            n /= 10;
            used += 1;
        }
        let start = 10 - used.max(width.min(10));
        tmp[start..].iter().for_each(|b| self.push(*b));
        self
    }

    pub fn send(mut self) {
        let n = self.len.min(ROOM - 1);
        self.buf[n] = b'\n';
        let _ = mk_debug(self.buf.as_ptr(), n + 1);
    }

    fn push(&mut self, b: u8) {
        if self.len < ROOM - 1 {
            self.buf[self.len] = b;
            self.len += 1;
        }
    }
}

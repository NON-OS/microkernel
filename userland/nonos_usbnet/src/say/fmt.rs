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

//! A log line built in place, with decimal and hexadecimal numbers.

pub struct Line {
    buf: [u8; 160],
    n: usize,
}

impl Line {
    pub(crate) fn new() -> Self {
        Self { buf: [0; 160], n: 0 }
    }

    pub fn put(&mut self, bytes: &[u8]) {
        for &b in bytes {
            if self.n == self.buf.len() - 1 {
                return;
            }
            self.buf[self.n] = b;
            self.n += 1;
        }
    }

    pub fn dec(&mut self, v: i64) {
        if v < 0 {
            self.put(b"-");
        }
        let mut digits = [0u8; 20];
        let (mut v, mut i) = (v.unsigned_abs(), digits.len());
        loop {
            i -= 1;
            digits[i] = b'0' + (v % 10) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        self.put(&digits[i..]);
    }

    /// `v` as `width` lowercase hex digits.
    pub fn hex(&mut self, v: u32, width: usize) {
        for shift in (0..width).rev() {
            self.put(&[b"0123456789abcdef"[(v >> (shift * 4)) as usize & 0xF]]);
        }
    }

    /// Out on the serial log, with its newline.
    pub fn send(mut self) {
        self.buf[self.n] = b'\n';
        let _ = nonos_libc::mk_debug(self.buf.as_ptr(), self.n + 1);
    }
}

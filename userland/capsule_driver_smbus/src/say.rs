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

//! One console line, cut to fit, with the numbers the survey prints.

use nonos_libc::mk_debug;

const DIGITS: &[u8; 16] = b"0123456789abcdef";

pub struct Line {
    buf: [u8; 120],
    n: usize,
}

impl Line {
    pub fn new() -> Self {
        let mut l = Self { buf: [0; 120], n: 0 };
        l.put(b"driver.smbus: ");
        l
    }

    pub fn put(&mut self, bytes: &[u8]) -> &mut Self {
        for &b in bytes {
            if self.n < self.buf.len() - 1 {
                self.buf[self.n] = b;
                self.n += 1;
            }
        }
        self
    }

    /// `0x` and `width` hex digits.
    pub fn hex(&mut self, v: u32, width: u32) -> &mut Self {
        self.put(b"0x");
        for i in (0..width).rev() {
            self.put(&[DIGITS[(v >> (i * 4)) as usize & 0xF]]);
        }
        self
    }

    pub fn send(&mut self) {
        self.buf[self.n] = b'\n';
        let _ = mk_debug(self.buf.as_ptr(), self.n + 1);
    }
}

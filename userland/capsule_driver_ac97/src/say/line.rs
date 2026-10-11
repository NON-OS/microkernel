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

//! One console line built without allocating, cut to fit and always ending
//! in a newline so the next line on the serial console starts on its own.

use nonos_libc::mk_debug;

pub struct Line {
    buf: [u8; 96],
    len: usize,
}

impl Line {
    pub fn new(start: &[u8]) -> Self {
        let mut line = Self { buf: [0; 96], len: 0 };
        line.push(start);
        line
    }

    pub fn push(&mut self, bytes: &[u8]) -> &mut Self {
        for &b in bytes.iter().take(self.buf.len() - 1 - self.len) {
            self.buf[self.len] = b;
            self.len += 1;
        }
        self
    }

    pub fn hex16(&mut self, v: u16) -> &mut Self {
        let digits = b"0123456789abcdef";
        let text: [u8; 4] = core::array::from_fn(|i| digits[(v >> (12 - 4 * i)) as usize & 0xF]);
        self.push(&text)
    }

    pub fn dec(&mut self, v: u32) -> &mut Self {
        let mut text = [0u8; 10];
        let mut at = text.len();
        let mut n = v;
        loop {
            at -= 1;
            text[at] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                return self.push(&text[at..]);
            }
        }
    }

    pub fn say(&mut self) {
        self.buf[self.len] = b'\n';
        let _ = mk_debug(self.buf.as_ptr(), self.len + 1);
    }
}

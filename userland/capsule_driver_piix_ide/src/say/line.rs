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
    buf: [u8; 112],
    len: usize,
}

impl Line {
    pub fn new(start: &[u8]) -> Self {
        let mut line = Self { buf: [0; 112], len: 0 };
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

    /// Printable ASCII only, so a device's text cannot break the console.
    pub fn text(&mut self, bytes: &[u8]) -> &mut Self {
        for &b in bytes {
            self.push(&[if (0x20..=0x7e).contains(&b) { b } else { b'?' }]);
        }
        self
    }

    pub fn dec(&mut self, v: u64) -> &mut Self {
        let mut text = [0u8; 20];
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

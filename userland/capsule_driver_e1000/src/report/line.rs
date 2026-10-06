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
    buf: [u8; 64],
    len: usize,
}

impl Line {
    pub fn new(start: &[u8]) -> Self {
        let mut line = Self { buf: [0; 64], len: 0 };
        line.push(start);
        line
    }

    pub fn push(&mut self, bytes: &[u8]) {
        for &b in bytes.iter().take(self.buf.len() - 1 - self.len) {
            self.buf[self.len] = b;
            self.len += 1;
        }
    }

    pub fn hex(&mut self, v: u32) {
        let digits = b"0123456789abcdef";
        let text: [u8; 8] = core::array::from_fn(|i| digits[(v >> (28 - 4 * i)) as usize & 0xF]);
        self.push(&text);
    }

    pub fn say(mut self) {
        self.buf[self.len] = b'\n';
        let _ = mk_debug(self.buf.as_ptr(), self.len + 1);
    }
}

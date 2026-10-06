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

//! One console line, built in place and sent whole through `mk_debug`, so a
//! line never interleaves with another capsule's. Text past the room is cut
//! and the line still ends in a newline.

use nonos_libc::mk_debug;

const ROOM: usize = 128;

pub struct Line {
    buf: [u8; ROOM],
    len: usize,
}

impl Line {
    pub fn new() -> Self {
        let mut line = Self { buf: [0u8; ROOM], len: 0 };
        line.text("igc: ");
        line
    }

    pub(super) fn byte(&mut self, b: u8) {
        if self.len < ROOM - 1 {
            self.buf[self.len] = b;
            self.len += 1;
        }
    }

    pub fn text(&mut self, s: &str) -> &mut Self {
        for &b in s.as_bytes() {
            self.byte(b);
        }
        self
    }

    pub fn send(&mut self) {
        self.buf[self.len] = b'\n';
        let _ = mk_debug(self.buf.as_ptr(), self.len + 1);
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

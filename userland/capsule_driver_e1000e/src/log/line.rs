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

//! Console lines, each starting `e1000e: `. A machine with no serial port is
//! read back with `log e1000e`, so every step that can fail names itself in
//! one of these. A line is built in a fixed buffer; text past its end is
//! dropped, never written beyond it.

const LINE_MAX: usize = 160;

pub struct Line {
    pub(super) buf: [u8; LINE_MAX],
    pub(super) len: usize,
}

impl Line {
    pub fn new() -> Self {
        let mut l = Line { buf: [0; LINE_MAX], len: 0 };
        l.text("e1000e: ");
        l
    }

    pub fn text(&mut self, t: &str) -> &mut Self {
        for &b in t.as_bytes() {
            self.byte(b);
        }
        self
    }

    pub fn send(&mut self) {
        let n = self.len.min(LINE_MAX - 1);
        self.buf[n] = b'\n';
        let _ = nonos_libc::mk_debug(self.buf.as_ptr(), n + 1);
    }

    pub(super) fn byte(&mut self, b: u8) {
        if self.len < LINE_MAX - 1 {
            self.buf[self.len] = b;
            self.len += 1;
        }
    }
}

/// One line: `e1000e: ` and `what`.
pub fn say(what: &str) {
    Line::new().text(what).send();
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

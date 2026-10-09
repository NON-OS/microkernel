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

//! One line for the kernel's log, `[AREA] what happened`, the line a failure
//! on the desktop leaves beside what it says on screen. MkDebug writes it to
//! the serial console, and the kernel keeps the console's last lines in
//! memory, where the Terminal's `log AREA` reads them on a laptop with no
//! serial port. Built on the stack, so it can be said when the heap could
//! not be had; pure but for `say`, so the host proofs hold the line.
//!
//! The line is one line: a byte that would break it (a newline, any control
//! byte) is written as a space. It is cut to `LINE_MAX`, never in the middle
//! of a character, and always ends in its newline. Only a capsule that holds
//! Debug is heard (a kernel built with capsule-serial-debug grants it to
//! those whose manifest asks); for any other the kernel refuses the line and
//! `say` reports that it was not written.

use nonos_libc::mk_debug;

/// Longest line, newline included; MkDebug takes at most 256 bytes.
pub const LINE_MAX: usize = 200;

/// A log line being built: `Line::new(b"LAUNCH").text(b"...").num(rc)`.
pub struct Line {
    buf: [u8; LINE_MAX],
    /* Bytes before the newline, which is always at `buf[len]`. */
    len: usize,
}

impl Line {
    /// `[AREA] `, the tag `log AREA` finds the line by.
    pub fn new(area: &[u8]) -> Self {
        let mut line = Line { buf: [0; LINE_MAX], len: 0 };
        line.buf[0] = b'\n';
        line.push(b"[");
        line.push(area);
        line.push(b"] ");
        line
    }

    /// `text`, with any byte that would end the line written as a space.
    pub fn text(mut self, text: &[u8]) -> Self {
        self.push(text);
        self
    }

    /// `v` in decimal.
    pub fn num(mut self, v: i64) -> Self {
        let mut digits = [0u8; 20];
        let mut i = digits.len();
        let mut n = v.unsigned_abs();
        loop {
            i -= 1;
            digits[i] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        if v < 0 {
            self.push(b"-");
        }
        self.push(&digits[i..]);
        self
    }

    /// The whole line, its newline included.
    pub fn bytes(&self) -> &[u8] {
        &self.buf[..self.len + 1]
    }

    fn push(&mut self, text: &[u8]) {
        let room = LINE_MAX - 1 - self.len;
        let mut take = text.len().min(room);
        if take < text.len() {
            /* Cut on a character: back off the bytes of one cut in two. */
            while take > 0 && text[take] & 0xC0 == 0x80 {
                take -= 1;
            }
        }
        for (slot, &b) in self.buf[self.len..self.len + take].iter_mut().zip(&text[..take]) {
            *slot = if b < 0x20 || b == 0x7F { b' ' } else { b };
        }
        self.len += take;
        self.buf[self.len] = b'\n';
    }
}

/// Write `line` to the kernel's log; false when the kernel refused it (the
/// caller holds no Debug), the one case it is not there for `log` to find.
pub fn say(line: &Line) -> bool {
    let bytes = line.bytes();
    mk_debug(bytes.as_ptr(), bytes.len()) >= 0
}

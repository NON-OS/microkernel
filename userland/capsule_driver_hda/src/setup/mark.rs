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
//! The `[HDA]` lines bring-up writes to the serial console, built on the
//! stack. They are what a boot log on a machine without a debugger shows.

use nonos_libc::mk_debug;

pub(crate) fn mark(s: &str) {
    mk_debug(s.as_ptr(), s.len());
}

/// One console line, cut to fit.
pub(crate) struct Line {
    buf: [u8; 120],
    n: usize,
}

impl Line {
    pub(crate) fn new(s: &str) -> Self {
        let mut l = Line { buf: [0; 120], n: 0 };
        l.s(s);
        l
    }

    pub(crate) fn s(&mut self, s: &str) -> &mut Self {
        for &b in s.as_bytes() {
            if self.n + 1 < self.buf.len() {
                self.buf[self.n] = b;
                self.n += 1;
            }
        }
        self
    }

    /// `v` in hex, `digits` long.
    pub(crate) fn hex(&mut self, v: u32, digits: u32) -> &mut Self {
        let mut shift = digits * 4;
        while shift > 0 {
            shift -= 4;
            let nib = ((v >> shift) & 0xf) as u8;
            let c = if nib < 10 { b'0' + nib } else { b'a' + nib - 10 };
            if self.n + 1 < self.buf.len() {
                self.buf[self.n] = c;
                self.n += 1;
            }
        }
        self
    }

    pub(crate) fn dec(&mut self, v: u32) -> &mut Self {
        let mut d = [0u8; 10];
        let mut k = 0usize;
        let mut r = v;
        loop {
            d[k] = b'0' + (r % 10) as u8;
            k += 1;
            r /= 10;
            if r == 0 {
                break;
            }
        }
        while k > 0 {
            k -= 1;
            if self.n + 1 < self.buf.len() {
                self.buf[self.n] = d[k];
                self.n += 1;
            }
        }
        self
    }

    pub(crate) fn emit(&mut self) {
        self.buf[self.n] = b'\n';
        mk_debug(self.buf.as_ptr(), self.n + 1);
    }
}

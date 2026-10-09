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

//! Log lines about hubs, tagged `[usb-hub]` so `log usb` finds them.

const LINE_MAX: usize = 192;

/// One line built from text and decimal numbers, cut at `LINE_MAX`.
pub struct Line {
    buf: [u8; LINE_MAX],
    n: usize,
}

impl Line {
    pub fn new() -> Self {
        Self { buf: [0; LINE_MAX], n: 0 }.text(b"[usb-hub] ")
    }

    pub fn text(mut self, s: &[u8]) -> Self {
        for &b in s {
            if self.n == LINE_MAX - 1 {
                break;
            }
            self.buf[self.n] = b;
            self.n += 1;
        }
        self
    }

    pub fn num(self, v: u32) -> Self {
        let mut digits = [0u8; 10];
        let (mut i, mut v) = (digits.len(), v);
        loop {
            i -= 1;
            digits[i] = b'0' + (v % 10) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        self.text(&digits[i..])
    }

    pub fn say(mut self) {
        self.buf[self.n] = b'\n';
        let _ = nonos_libc::mk_debug(self.buf.as_ptr(), self.n + 1);
    }
}

/// The start of a line about `port` of the hub in `slot`.
pub fn hub_port(slot: u8, port: u8) -> Line {
    Line::new().text(b"hub slot ").num(slot as u32).text(b" port ").num(port as u32).text(b": ")
}

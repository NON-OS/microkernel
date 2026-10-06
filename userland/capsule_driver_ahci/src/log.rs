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

//! Bring-up lines on the serial console, each starting `[ahci]`. The driver
//! has only ever run on QEMU; on a first boot on real hardware these few
//! lines per controller and port (CAP, PI, and each port's status registers
//! after its bring-up) are what a failure is read from. A line is built in a
//! fixed buffer; text past its end is dropped, never written beyond it.

const LINE_MAX: usize = 160;

pub struct Line {
    buf: [u8; LINE_MAX],
    len: usize,
}

impl Line {
    pub fn new() -> Self {
        let mut l = Line { buf: [0; LINE_MAX], len: 0 };
        l.text(b"[ahci] ");
        l
    }

    pub fn text(&mut self, t: &[u8]) -> &mut Self {
        let room = LINE_MAX.saturating_sub(self.len).saturating_sub(1);
        let n = t.len().min(room);
        self.buf[self.len..self.len + n].copy_from_slice(&t[..n]);
        self.len += n;
        self
    }

    pub fn hex(&mut self, v: u32) -> &mut Self {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        let mut d = [0u8; 10];
        d[0] = b'0';
        d[1] = b'x';
        for (i, b) in d[2..].iter_mut().enumerate() {
            *b = DIGITS[((v >> (28 - i * 4)) & 0xF) as usize];
        }
        self.text(&d)
    }

    pub fn num(&mut self, mut v: u64) -> &mut Self {
        let mut d = [0u8; 20];
        let mut i = d.len();
        loop {
            i -= 1;
            d[i] = b'0' + (v % 10) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        self.text(&d[i..])
    }

    pub fn send(&mut self) {
        self.buf[self.len] = b'\n';
        let _ = nonos_libc::mk_debug(self.buf.as_ptr(), self.len + 1);
    }
}

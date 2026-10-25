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

//! A report written into a fixed buffer. Once a write does not fit, the sink
//! is full and takes nothing more, so a report is either whole or marked cut.

pub struct Sink<'a> {
    buf: &'a mut [u8],
    len: usize,
    full: bool,
}

impl<'a> Sink<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, len: 0, full: false }
    }

    pub fn bytes(&mut self, b: &[u8]) -> &mut Self {
        if self.full || self.buf.len() - self.len < b.len() {
            self.full = true;
            return self;
        }
        self.buf[self.len..self.len + b.len()].copy_from_slice(b);
        self.len += b.len();
        self
    }

    pub fn dec(&mut self, mut v: u64) -> &mut Self {
        let (mut d, mut i) = ([0u8; 20], 20);
        while i == d.len() || v > 0 {
            i -= 1;
            d[i] = b'0' + (v % 10) as u8;
            v /= 10;
        }
        self.bytes(&d[i..])
    }

    /// `v` as `digits` lowercase hex digits, at most sixteen.
    pub fn hex(&mut self, v: u64, digits: usize) -> &mut Self {
        for i in (0..digits.min(16)).rev() {
            self.bytes(&[b"0123456789abcdef"[((v >> (i * 4)) & 0xF) as usize]]);
        }
        self
    }

    pub fn len(&self) -> usize {
        self.len
    }
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Whether a write was refused for want of room.
    pub fn full(&self) -> bool {
        self.full
    }
    pub fn room(&self) -> usize {
        self.buf.len() - self.len
    }
    pub fn written(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

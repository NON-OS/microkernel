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

//! A fixed on-stack string for values the panel formats at paint time. The paint
//! path runs on every frame, so it formats into this rather than allocating.

pub const CAP: usize = 72;

pub struct ValBuf {
    pub(super) bytes: [u8; CAP],
    pub(super) len: usize,
}

impl Default for ValBuf {
    fn default() -> Self {
        Self::new()
    }
}

impl ValBuf {
    pub fn new() -> Self {
        ValBuf { bytes: [0; CAP], len: 0 }
    }

    /// Append `s`, stopping before a character that no longer fits whole:
    /// a buffer cut inside one would not read back as text at all.
    pub fn push_str(&mut self, s: &str) {
        for ch in s.chars() {
            if self.len + ch.len_utf8() > CAP {
                return;
            }
            for &b in ch.encode_utf8(&mut [0u8; 4]).as_bytes() {
                self.push(b);
            }
        }
    }

    pub fn push_bytes(&mut self, s: &[u8]) {
        for b in s {
            self.push(if b.is_ascii_graphic() || *b == b' ' { *b } else { b'?' });
        }
    }

    /// Append `v` in decimal.
    pub fn push_dec(&mut self, mut v: u32) {
        let mut digits = [0u8; 10];
        let mut i = digits.len();
        loop {
            i -= 1;
            digits[i] = b'0' + (v % 10) as u8;
            v /= 10;
            if v == 0 {
                break;
            }
        }
        for &d in &digits[i..] {
            self.push(d);
        }
    }

    /// Append `v` as four lowercase hex digits.
    pub fn push_hex16(&mut self, v: u16) {
        const DIGITS: &[u8; 16] = b"0123456789abcdef";
        for shift in [12u32, 8, 4, 0] {
            self.push(DIGITS[((v >> shift) & 0xF) as usize]);
        }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.bytes[..self.len]).unwrap_or("")
    }

    pub(super) fn push(&mut self, b: u8) {
        if self.len < CAP {
            self.bytes[self.len] = b;
            self.len += 1;
        }
    }
}

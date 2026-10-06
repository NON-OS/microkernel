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

//! A short line of ASCII, built without the heap.

/// Room for the longest boot log line: label, state word and detail.
const TEXT_LEN: usize = 96;

#[derive(Clone, Copy)]
pub struct Text {
    buf: [u8; TEXT_LEN],
    len: usize,
}

impl Text {
    pub const fn new() -> Self {
        Self { buf: [0u8; TEXT_LEN], len: 0 }
    }

    /// Append `s`, dropping whatever does not fit.
    pub fn push(mut self, s: &[u8]) -> Self {
        let n = s.len().min(TEXT_LEN.saturating_sub(self.len));
        self.buf[self.len..self.len + n].copy_from_slice(&s[..n]);
        self.len += n;
        self
    }

    pub fn dec(self, mut n: u64) -> Self {
        let mut digits = [0u8; 20];
        let mut at = digits.len();
        while at > 0 {
            at -= 1;
            digits[at] = b'0' + (n % 10) as u8;
            n /= 10;
            if n == 0 {
                break;
            }
        }
        self.push(&digits[at..])
    }

    /// Lowercase hex, two digits a byte.
    pub fn hex(self, bytes: &[u8]) -> Self {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        bytes.iter().fold(self, |t, &b| t.push(&[HEX[(b >> 4) as usize], HEX[(b & 15) as usize]]))
    }

    /// A size in whole KB, or in bytes below one KB.
    pub fn size(self, n: usize) -> Self {
        if n < 1024 {
            self.dec(n as u64).push(b" B")
        } else {
            self.dec((n.saturating_add(512) / 1024) as u64).push(b" KB")
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

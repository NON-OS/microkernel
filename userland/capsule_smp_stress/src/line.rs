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

//! A report line built in a fixed buffer: the capsule has no heap. Pure, so
//! the host proofs can hold what it prints.

/// Under the kernel's 256-byte limit for one debug line.
pub const CAP: usize = 240;

pub struct Line {
    buf: [u8; CAP],
    len: usize,
}

impl Line {
    pub fn tagged() -> Self {
        let mut line = Line { buf: [0; CAP], len: 0 };
        line.str(b"[SMP-STRESS]");
        line
    }

    /// Append `s`; what does not fit is dropped, never written past the end.
    pub fn str(&mut self, s: &[u8]) -> &mut Self {
        let n = s.len().min(CAP - self.len);
        self.buf[self.len..self.len + n].copy_from_slice(&s[..n]);
        self.len += n;
        self
    }

    /// Append ` key=value` with the value in decimal.
    pub fn field(&mut self, key: &[u8], value: u64) -> &mut Self {
        self.str(b" ").str(key).str(b"=").dec(value)
    }

    pub fn dec(&mut self, mut value: u64) -> &mut Self {
        let mut digits = [0u8; 20];
        let mut at = digits.len();
        loop {
            at -= 1;
            digits[at] = b'0' + (value % 10) as u8;
            value /= 10;
            if value == 0 {
                break;
            }
        }
        self.str(&digits[at..])
    }

    pub fn bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

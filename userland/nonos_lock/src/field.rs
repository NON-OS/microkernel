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
//! A passphrase as it is typed: at most `FIELD_MAX` bytes of UTF-8 in a
//! fixed buffer, wiped on every clear and when it is let go.

use crate::wipe::secure_zero;

pub const FIELD_MAX: usize = crate::rules::MAX_BYTES;

pub struct Field {
    bytes: [u8; FIELD_MAX],
    len: usize,
}

impl Default for Field {
    fn default() -> Self {
        Field { bytes: [0; FIELD_MAX], len: 0 }
    }
}

impl Field {
    /// Add `c` at the end; false when it does not fit.
    pub fn push(&mut self, c: char) -> bool {
        let mut buf = [0u8; 4];
        let s = c.encode_utf8(&mut buf);
        let n = s.len();
        if self.len + n > FIELD_MAX {
            secure_zero(&mut buf);
            return false;
        }
        self.bytes[self.len..self.len + n].copy_from_slice(&buf[..n]);
        self.len += n;
        secure_zero(&mut buf);
        true
    }

    /// Take the last character away.
    pub fn pop(&mut self) {
        let mut at = self.len;
        while at > 0 {
            at -= 1;
            // A UTF-8 character starts on a byte that is not 10xxxxxx.
            if self.bytes[at] & 0xC0 != 0x80 {
                break;
            }
        }
        secure_zero(&mut self.bytes[at..self.len]);
        self.len = at;
    }

    pub fn clear(&mut self) {
        secure_zero(&mut self.bytes[..self.len]);
        self.len = 0;
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// The characters typed, which the field shows as dots.
    pub fn chars(&self) -> usize {
        self.bytes().iter().filter(|&&b| b & 0xC0 != 0x80).count()
    }
}

impl Drop for Field {
    fn drop(&mut self) {
        secure_zero(&mut self.bytes);
    }
}

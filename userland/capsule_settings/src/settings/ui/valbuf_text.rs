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

//! Text into the buffer: a string cut only between characters, and bytes
//! with anything unprintable shown as '?'.

use super::valbuf::{ValBuf, CAP};

impl ValBuf {
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
}

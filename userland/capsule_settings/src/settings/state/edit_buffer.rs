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

use nonos_app_skeleton::input::text::{last_char_len, push_char};

use super::cache::STRING_CAP;

#[derive(Clone, Copy)]
pub struct EditBuffer {
    pub bytes: [u8; STRING_CAP],
    pub len: usize,
}

impl EditBuffer {
    pub const fn empty() -> Self {
        Self { bytes: [0u8; STRING_CAP], len: 0 }
    }

    pub fn push(&mut self, b: u8) -> bool {
        if self.len >= STRING_CAP {
            return false;
        }
        self.bytes[self.len] = b;
        self.len += 1;
        true
    }

    /// Append `ch` whole, within `cap` bytes, or not at all: a value cut
    /// inside a character is not text.
    pub fn push_char(&mut self, ch: char, cap: usize) -> bool {
        match push_char(&mut self.bytes, self.len, cap, ch) {
            Some(len) => {
                self.len = len;
                true
            }
            None => false,
        }
    }

    /// Remove the last character, all of its bytes. The buffer holds UTF-8,
    /// and taking one byte of a two byte letter left half of it behind.
    pub fn pop(&mut self) -> bool {
        let n = last_char_len(self.as_slice());
        if n == 0 {
            return false;
        }
        self.bytes[self.len - n..self.len].fill(0);
        self.len -= n;
        true
    }

    /// The characters held.
    pub fn char_count(&self) -> usize {
        core::str::from_utf8(self.as_slice()).map_or(self.len, |s| s.chars().count())
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

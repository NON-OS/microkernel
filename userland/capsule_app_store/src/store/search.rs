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

//! The query, and what it matches.

use alloc::vec::Vec;

/// Longer than any name listed, so a held key cannot grow the field.
const MAX: usize = 64;

#[derive(Default)]
pub struct Search {
    pub active: bool,
    text: Vec<u8>,
}

impl Search {
    pub fn text(&self) -> &[u8] {
        &self.text
    }

    /// Keeps what is typed: reopening to add a letter should not
    /// discard the word.
    pub fn open(&mut self) {
        self.active = true;
    }

    /// Leave and clear: a closed field that went on filtering would
    /// hide rows with nothing on screen to say why.
    pub fn close(&mut self) {
        self.active = false;
        self.text.clear();
    }

    pub fn push(&mut self, byte: u8) -> bool {
        if self.text.len() >= MAX {
            return false;
        }
        self.text.push(byte.to_ascii_lowercase());
        true
    }

    pub fn pop(&mut self) -> bool {
        self.text.pop().is_some()
    }

    /// Case-insensitive substring. Empty accepts everything; longer
    /// than the name matches nothing rather than panicking.
    pub fn accepts(&self, name: &[u8]) -> bool {
        if self.text.is_empty() {
            return true;
        }
        if self.text.len() > name.len() {
            return false;
        }
        let lower = |b: &u8| b.to_ascii_lowercase();
        name.windows(self.text.len()).any(|w| w.iter().map(lower).eq(self.text.iter().copied()))
    }
}

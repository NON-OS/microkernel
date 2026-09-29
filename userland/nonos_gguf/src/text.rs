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

//! A short string from the header, such as the model's architecture or name,
//! kept whole when it fits and marked when it does not.

pub const TEXT_BYTES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Text {
    bytes: [u8; TEXT_BYTES],
    kept: usize,
    /// The string's length in the file, which may be more than was kept.
    pub full_len: u64,
}

impl Text {
    pub(crate) fn empty(full_len: u64) -> Self {
        Text { bytes: [0; TEXT_BYTES], kept: 0, full_len }
    }

    pub(crate) fn keep(&mut self, i: u64, b: u8) {
        if i < TEXT_BYTES as u64 {
            self.bytes[i as usize] = b;
            self.kept = i as usize + 1;
        }
    }

    /// The bytes kept, the whole string when `is_whole`.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes[..self.kept]
    }

    pub fn is_whole(&self) -> bool {
        self.kept as u64 == self.full_len
    }
}

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

//! The line buffer itself: the driver's tag, appended text, and the newline.

/// The kernel takes at most 256 bytes per debug call; a line stays well
/// under that and always ends in a newline.
pub const LINE_BYTES: usize = 160;

pub struct Line {
    buf: [u8; LINE_BYTES],
    len: usize,
}

impl Line {
    /// A line that starts with the driver's tag.
    pub fn new() -> Self {
        let mut line = Self { buf: [0; LINE_BYTES], len: 0 };
        line.text(b"nvme: ");
        line
    }

    /// Append bytes, cut at the room left for the newline.
    pub fn text(&mut self, s: &[u8]) -> &mut Self {
        for &b in s {
            if self.len == LINE_BYTES - 1 {
                break;
            }
            self.buf[self.len] = b;
            self.len += 1;
        }
        self
    }

    /// The line as the console gets it, newline included.
    pub fn finish(&mut self) -> &[u8] {
        self.buf[self.len] = b'\n';
        &self.buf[..self.len + 1]
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

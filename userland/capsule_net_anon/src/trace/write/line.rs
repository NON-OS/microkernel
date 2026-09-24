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

//! A fixed width log line being built, with no allocation on the path.

/// Longest line this will emit. Anything past it is dropped rather than
/// wrapped, because a torn line is worse to read than a short one.
pub const LINE_MAX: usize = 96;

/// A line under construction.
pub struct Line {
    pub(super) buf: [u8; LINE_MAX],
    pub(super) at: usize,
}

impl Line {
    /// Start a line under the shared prefix.
    ///
    pub fn new(stage: &[u8]) -> Self {
        let mut line = Self { buf: [0u8; LINE_MAX], at: 0 };
        line.text(b"[ANON] ");
        line.text(stage);
        line
    }

    /// Append text, stopping at the line limit rather than wrapping.
    pub fn text(&mut self, s: &[u8]) -> &mut Self {
        for &b in s {
            if self.at >= LINE_MAX - 1 {
                return self;
            }
            self.buf[self.at] = b;
            self.at += 1;
        }
        self
    }

    /// The finished line, newline included.
    pub fn finish(&mut self) -> (&[u8], usize) {
        self.buf[self.at] = b'\n';
        (&self.buf, self.at + 1)
    }
}

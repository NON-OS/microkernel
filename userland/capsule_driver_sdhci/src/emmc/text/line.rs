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

//! The line buffer, its prefix and raw byte append.

pub const LINE_MAX: usize = 160;
pub const PREFIX: &[u8] = b"driver.emmc: ";

pub struct Line {
    buf: [u8; LINE_MAX],
    n: usize,
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

impl Line {
    /// A line that starts with `driver.emmc: `.
    pub fn new() -> Self {
        let mut l = Self { buf: [0; LINE_MAX], n: 0 };
        l.s(PREFIX);
        l
    }

    /// Append bytes; what does not fit is cut.
    pub fn s(&mut self, b: &[u8]) -> &mut Self {
        for &c in b {
            if self.n == LINE_MAX {
                break;
            }
            self.buf[self.n] = c;
            self.n += 1;
        }
        self
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.buf[..self.n]
    }
}

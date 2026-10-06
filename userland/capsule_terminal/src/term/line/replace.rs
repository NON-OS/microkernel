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

use super::types::Line;
use crate::term::dimensions::LINE_MAX;

impl Line {
    pub fn replace(&mut self, src: &[u8]) {
        // A source longer than the line is cut at a character boundary, so
        // the line never ends in part of a character.
        let mut n = src.len().min(LINE_MAX);
        while n < src.len() && n > 0 && src[n] & 0xC0 == 0x80 {
            n -= 1;
        }
        self.buf[..n].copy_from_slice(&src[..n]);
        self.len = n;
        self.cursor = n;
    }
}

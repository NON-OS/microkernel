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

//! Where the characters start. The line holds UTF-8, so one step of the
//! cursor, Backspace or Delete is one character of one to four bytes; a step
//! of one byte inside a character split it and left bytes no font can draw.

use super::types::Line;

fn continuation(b: u8) -> bool {
    b & 0xC0 == 0x80
}

impl Line {
    /// The start of the character before `at`.
    pub(super) fn prev_boundary(&self, at: usize) -> usize {
        let mut i = at.min(self.len).saturating_sub(1);
        while i > 0 && continuation(self.buf[i]) {
            i -= 1;
        }
        i
    }

    /// The end of the character that starts at `at`.
    pub(super) fn next_boundary(&self, at: usize) -> usize {
        let mut i = (at + 1).min(self.len);
        while i < self.len && continuation(self.buf[i]) {
            i += 1;
        }
        i
    }
}

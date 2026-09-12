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

//! Moving by word.
//!
//! Home and End reach the two ends of a line and the arrows step one byte.
//! Between those is every real edit: a path halfway along a long command, the
//! flag before it. Without word motion the only way there is to hold an arrow
//! down and watch, which is why Ctrl with the arrows means this in every shell.
//!
//! The boundary rule matches `delete_word`, so Ctrl-Left and Ctrl-W land in
//! the same place and Ctrl-W is always "delete what Ctrl-Left would skip".

use super::types::Line;

impl Line {
    /// Move to the start of the word before the cursor.
    ///
    /// Skips any run of spaces first, so pressing it at the end of `ls -la /x`
    /// lands on `/x` rather than stopping in the gap in front of it.
    pub fn move_word_left(&mut self) -> bool {
        if self.cursor == 0 {
            return false;
        }
        let mut at = self.cursor;
        while at > 0 && self.buf[at - 1] == b' ' {
            at -= 1;
        }
        while at > 0 && self.buf[at - 1] != b' ' {
            at -= 1;
        }
        self.cursor = at;
        true
    }

    /// Move to the start of the word after the cursor.
    ///
    /// Steps over the current word and then the gap, landing on the next
    /// word's first character rather than on the space in front of it, which
    /// is where the next thing anyone types belongs.
    pub fn move_word_right(&mut self) -> bool {
        if self.cursor >= self.len {
            return false;
        }
        let mut at = self.cursor;
        while at < self.len && self.buf[at] != b' ' {
            at += 1;
        }
        while at < self.len && self.buf[at] == b' ' {
            at += 1;
        }
        self.cursor = at;
        true
    }
}

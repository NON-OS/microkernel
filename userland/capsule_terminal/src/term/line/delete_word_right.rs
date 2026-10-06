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

impl Line {
    /*
     * Delete the word after the cursor (Alt-D): the run Alt-F would step
     * over, word first and then the gap, so the two keys agree.
     */
    pub fn delete_word_right(&mut self) -> bool {
        let mut end = self.cursor;
        while end < self.len && self.buf[end] != b' ' {
            end += 1;
        }
        while end < self.len && self.buf[end] == b' ' {
            end += 1;
        }
        let removed = end - self.cursor;
        if removed == 0 {
            return false;
        }
        let mut cut = [0u8; super::types::KILL_CAP];
        cut[..removed].copy_from_slice(&self.buf[self.cursor..end]);
        self.hold_killed(&cut[..removed]);
        self.buf.copy_within(end..self.len, self.cursor);
        self.len -= removed;
        true
    }
}

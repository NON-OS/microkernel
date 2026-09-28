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

//! Drawing a character at the cursor.

use super::state::Term;
use crate::width::{is_emoji_modifier, width, ZWJ};

impl Term {
    pub(super) fn print_char(&mut self, c: char) {
        let c = if (c as u32) < 0x80 {
            self.charsets.map(c)
        } else {
            self.charsets.single = None;
            c
        };
        /*
         * A joiner, a skin tone after an emoji, and nonspacing marks are
         * drawn over the character before them, not in a cell of their own.
         */
        if core::mem::take(&mut self.after_zwj) && self.attach_mark(c) {
            return;
        }
        if c == ZWJ {
            self.attach_mark(c);
            self.after_zwj = true;
            return;
        }
        if is_emoji_modifier(c)
            && self.prev_cell().is_some_and(|p| p.is_wide())
            && self.attach_mark(c)
        {
            return;
        }
        match width(c) {
            0 => {
                self.attach_mark(c);
            }
            w => {
                self.put_glyph(c, w);
                self.last_char = Some(c);
            }
        }
    }
}

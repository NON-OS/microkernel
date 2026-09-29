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

//! DECSCUSR: the cursor shape and whether it blinks.

use super::state::Term;
use super::types::CursorShape;

impl Term {
    /// DECSCUSR: 0 and 1 blinking block, 2 steady block, 3 and 4 underline,
    /// 5 and 6 bar, odd numbers blinking.
    pub(super) fn set_cursor_style(&mut self, v: u16) {
        let (shape, blink) = match v {
            0 | 1 => (CursorShape::Block, true),
            2 => (CursorShape::Block, false),
            3 => (CursorShape::Underline, true),
            4 => (CursorShape::Underline, false),
            5 => (CursorShape::Bar, true),
            6 => (CursorShape::Bar, false),
            _ => return,
        };
        self.cursor_shape = shape;
        self.modes.cursor_blink = blink;
        let y = self.scr_ref().cur.y;
        self.touch(y);
    }
}

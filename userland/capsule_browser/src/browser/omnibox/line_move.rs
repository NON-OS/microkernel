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

use super::line_edit::LineEdit;
use super::word::{next_char, next_word, prev_char, prev_word};

/* Caret movement. With `extend` the anchor stays put and the selection
 * grows; without it a plain arrow first collapses a selection to its edge. */
impl LineEdit {
    pub fn left(&mut self, word: bool, extend: bool) {
        let to = if word {
            prev_word(&self.text, self.caret)
        } else if self.has_selection() && !extend {
            self.selection().0
        } else {
            prev_char(&self.text, self.caret)
        };
        self.move_to(to, extend);
    }

    pub fn right(&mut self, word: bool, extend: bool) {
        let to = if word {
            next_word(&self.text, self.caret)
        } else if self.has_selection() && !extend {
            self.selection().1
        } else {
            next_char(&self.text, self.caret)
        };
        self.move_to(to, extend);
    }

    pub fn home(&mut self, extend: bool) {
        self.move_to(0, extend);
    }

    pub fn end(&mut self, extend: bool) {
        self.move_to(self.text.len(), extend);
    }

    fn move_to(&mut self, to: usize, extend: bool) {
        self.caret = to;
        if !extend {
            self.anchor = to;
        }
    }
}

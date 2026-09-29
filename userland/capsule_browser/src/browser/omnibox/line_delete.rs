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

/* Deletion keys. Each removes the selection when there is one, and
 * otherwise the character or word on its side of the caret. */
impl LineEdit {
    pub fn backspace(&mut self) {
        let to = prev_char(&self.text, self.caret);
        self.remove_to(to);
    }

    pub fn delete(&mut self) {
        let to = next_char(&self.text, self.caret);
        self.remove_to(to);
    }

    pub fn backspace_word(&mut self) {
        let to = prev_word(&self.text, self.caret);
        self.remove_to(to);
    }

    pub fn delete_word(&mut self) {
        let to = next_word(&self.text, self.caret);
        self.remove_to(to);
    }

    fn remove_to(&mut self, to: usize) {
        if !self.has_selection() {
            self.anchor = to;
        }
        self.replace_selection("");
    }
}

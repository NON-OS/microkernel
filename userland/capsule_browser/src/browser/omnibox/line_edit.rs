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

use alloc::string::String;

/* Longest address the bar holds, in bytes. A paste beyond it is cut on a
 * character boundary rather than growing the buffer without bound. */
pub const MAX_LEN: usize = 2048;

/* One line of editable text. The caret and the selection anchor are byte
 * offsets that always sit on character boundaries; the selection runs
 * between them, and when they are equal nothing is selected. */
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LineEdit {
    pub text: String,
    pub caret: usize,
    pub anchor: usize,
}

impl LineEdit {
    pub fn new() -> Self {
        LineEdit::default()
    }

    /* Replace the whole text and park the caret at its end. */
    pub fn set(&mut self, text: &str) {
        self.text.clear();
        self.caret = 0;
        self.anchor = 0;
        self.replace_selection(text);
    }

    pub fn select_all(&mut self) {
        self.anchor = 0;
        self.caret = self.text.len();
    }

    pub fn caret_to_end(&mut self) {
        self.caret = self.text.len();
        self.anchor = self.caret;
    }

    /* Type or paste over the selection; the caret lands after the insert. */
    pub fn replace_selection(&mut self, s: &str) {
        let (a, b) = self.selection();
        let room = MAX_LEN.saturating_sub(self.text.len() - (b - a));
        let mut take = s.len().min(room);
        while !s.is_char_boundary(take) {
            take -= 1;
        }
        self.text.replace_range(a..b, &s[..take]);
        self.caret = a + take;
        self.anchor = self.caret;
    }
}

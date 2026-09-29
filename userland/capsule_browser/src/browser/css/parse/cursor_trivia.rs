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

use crate::browser::css::selector::{is_space, Comb};

use super::cursor::Cur;

/* What separates tokens without being one: comments, and whitespace. */
impl Cur<'_> {
    /* Comments only. A comment separates tokens without being whitespace,
     * so a comment between two type selectors leaves one bad compound. */
    pub fn comments(&mut self) {
        while self.peek() == Some(b'/') && self.at(1) == Some(b'*') {
            self.i = match self.s[self.i + 2..].find("*/") {
                Some(e) => self.i + e + 4,
                None => self.s.len(),
            };
        }
    }

    /* Whitespace and comments; true when any whitespace was crossed. */
    pub fn trivia(&mut self) -> bool {
        let mut ws = false;
        loop {
            self.comments();
            match self.peek() {
                Some(b) if is_space(b) => {
                    ws = true;
                    self.i += 1;
                }
                _ => return ws,
            }
        }
    }

    /* A combinator character, consumed: '>', '+' or '~'. */
    pub fn combinator(&mut self) -> Option<Comb> {
        let k = match self.peek()? {
            b'>' => Comb::Child,
            b'+' => Comb::NextSibling,
            b'~' => Comb::SubsequentSibling,
            _ => return None,
        };
        self.i += 1;
        Some(k)
    }
}

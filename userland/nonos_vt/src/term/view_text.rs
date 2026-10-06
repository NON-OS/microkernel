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

//! The visible rows as plain text, for self-tests, screen readers and
//! whatever else needs what is shown rather than how.

use alloc::string::String;
use alloc::vec::Vec;

use super::state::Term;

impl Term {
    /// Visible row `y` as text, marks included, trailing blanks dropped.
    pub fn row_text(&self, y: usize) -> String {
        let line = self.visible_line(y);
        let mut s = String::new();
        for x in 0..self.cols {
            let c = line.cell(x);
            if c.is_tail() {
                continue;
            }
            s.push(c.ch);
            if let Some(m) = line.marks_of(&c) {
                s.push_str(m);
            }
        }
        let n = s.trim_end().len();
        s.truncate(n);
        s
    }

    /// Every visible row, as `row_text` gives it.
    pub fn screen_text(&self) -> Vec<String> {
        (0..self.rows).map(|y| self.row_text(y)).collect()
    }
}

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

//! Absolute line numbers: every line the terminal ever had, counted from
//! its first, so a position in history survives scrolling.

use super::state::Term;
use super::types::Pos;
use crate::line::Line;

impl Term {
    /// The oldest line still held, and the last line of the screen.
    pub fn first_line(&self) -> u64 {
        self.scrolled - self.scrollback.len() as u64
    }

    pub fn last_line(&self) -> u64 {
        self.scrolled + self.rows as u64 - 1
    }

    pub fn line_at(&self, abs: u64) -> Option<&Line> {
        if abs < self.first_line() {
            return None;
        }
        if abs < self.scrolled {
            return self.scrollback.get((abs - self.first_line()) as usize);
        }
        let y = usize::try_from(abs - self.scrolled).ok()?;
        self.scr_ref().lines.get(y)
    }

    /// The absolute line shown on visible row `row`.
    pub fn abs_of_row(&self, row: usize) -> u64 {
        if self.alt_active {
            return self.scrolled + row as u64;
        }
        (self.scrolled + row as u64).saturating_sub(self.view as u64)
    }

    pub fn cursor_pos(&self) -> Pos {
        let s = self.scr_ref();
        Pos { line: self.scrolled + s.cur.y as u64, col: s.cur.x }
    }
}

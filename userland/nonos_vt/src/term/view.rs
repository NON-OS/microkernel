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

//! What the host draws: the visible rows, the cursor, which rows changed,
//! and each cell's final colours.

use super::state::Term;
use crate::line::Line;

impl Term {
    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn history_len(&self) -> usize {
        self.scrollback.len()
    }

    pub fn view_offset(&self) -> usize {
        self.view
    }

    /// Row `row` of what is on view, history included when scrolled back.
    pub fn visible_line(&self, row: usize) -> &Line {
        let row = row.min(self.rows - 1);
        if self.alt_active {
            return &self.alt.lines[row];
        }
        let sb = self.scrollback.len();
        let i = sb + row - self.view.min(sb);
        if i < sb {
            &self.scrollback[i]
        } else {
            &self.primary.lines[i - sb]
        }
    }

    /// Scroll the view: positive goes back into history. The alternate
    /// screen has none.
    pub fn scroll_view(&mut self, delta: isize) {
        if self.alt_active {
            return;
        }
        let max = self.scrollback.len() as isize;
        let v = (self.view as isize).saturating_add(delta).clamp(0, max) as usize;
        if v != self.view {
            self.view = v;
            self.touch_all();
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        self.scroll_view(isize::MIN);
    }
}

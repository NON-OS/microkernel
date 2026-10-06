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

//! Changing the screen size. The normal screen re-wraps its text to the new
//! width; the alternate screen is cut or padded, since the program drawing
//! on it redraws when told of the new size.

use alloc::vec;

use super::state::Term;
use crate::cell::Cell;
use crate::limits::{MAX_COLS, MAX_ROWS, MIN_COLS, MIN_ROWS};
use crate::line::Line;

pub(super) fn clamp_size(cols: usize, rows: usize) -> (usize, usize) {
    (cols.clamp(MIN_COLS, MAX_COLS), rows.clamp(MIN_ROWS, MAX_ROWS))
}

impl Term {
    pub fn resize(&mut self, cols: usize, rows: usize) {
        let (cols, rows) = clamp_size(cols, rows);
        if cols == self.cols && rows == self.rows {
            return;
        }
        self.reflow_primary(cols, rows);
        self.resize_alt(cols, rows);
        let old = self.tabs.len();
        self.tabs.resize(cols, false);
        for x in old..cols {
            self.tabs[x] = x % 8 == 0;
        }
        self.cols = cols;
        self.rows = rows;
        self.dirty = vec![true; rows];
        self.view = 0;
    }

    fn resize_alt(&mut self, cols: usize, rows: usize) {
        let s = &mut self.alt;
        for line in s.lines.iter_mut() {
            line.resize(cols, Cell::BLANK);
        }
        let len = s.lines.len();
        if rows < len {
            // Keep the cursor's row on screen: lose lines above it first.
            let above = (s.cur.y + 1).saturating_sub(rows).min(len - rows);
            s.lines.drain(..above);
            s.cur.y -= above;
            s.lines.truncate(rows);
        }
        while s.lines.len() < rows {
            s.lines.push(Line::new(cols, Cell::BLANK));
        }
        s.cur.x = s.cur.x.min(cols - 1);
        s.cur.y = s.cur.y.min(rows - 1);
        s.cur.pending_wrap = false;
        s.reset_region();
    }
}

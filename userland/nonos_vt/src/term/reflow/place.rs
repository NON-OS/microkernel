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

//! Choosing which re-wrapped lines are the screen, and putting everything
//! back.

use alloc::vec::Vec;

use super::placed::Placed;
use super::rewrap_all::rewrap_all;
use crate::cell::Cell;
use crate::line::Line;
use crate::term::Term;

/// The first line of the new screen. The old top row stays on top unless
/// the cursor would fall off the foot. Growing while at the foot, as after
/// a command's output, brings back as many history lines as rows were
/// added. Whatever the rules choose, the cursor ends on screen.
fn screen_start(top_out: usize, at: &Placed, rows: usize, old_rows: usize, cur_y: usize) -> usize {
    let mut start = top_out.min(at.line);
    if rows > old_rows && cur_y + 1 == old_rows {
        start = start.saturating_sub(rows - old_rows);
    }
    if at.line >= start + rows {
        start = at.line + 1 - rows;
    }
    start
}

impl Term {
    pub(in crate::term) fn reflow_primary(&mut self, cols: usize, rows: usize) {
        let base = self.first_line();
        let cur_y = self.primary.cur.y;
        let g = self.gather();
        let (mut out, at, top_out) = rewrap_all(&g, cols, self.cols);
        let start = screen_start(top_out, &at, rows, self.rows, cur_y);
        let mut screen: Vec<Line> = out.drain(start..).take(rows).collect();
        screen.resize(rows, Line::new(cols, Cell::BLANK));
        for mut line in out {
            line.trim();
            self.scrollback.push_back(line);
        }
        self.scrolled = base + self.scrollback.len() as u64;
        while self.scrollback.len() > self.scrollback_limit {
            self.scrollback.pop_front();
        }
        let s = &mut self.primary;
        s.lines = screen;
        s.cur.y = at.line - start;
        s.cur.x = at.col.min(cols - 1);
        s.cur.pending_wrap = at.pending;
        s.reset_region();
        self.prompts.clear();
    }
}

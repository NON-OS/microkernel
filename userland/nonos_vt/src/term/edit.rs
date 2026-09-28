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

//! Inserting and deleting characters and lines in place.

use super::state::Term;
use crate::limits::MAX_REPEAT;
use crate::width::width;

impl Term {
    pub(super) fn insert_chars(&mut self, n: usize) {
        let scr = self.scr();
        let (x, y) = (scr.cur.x, scr.cur.y);
        let blank = scr.blank();
        scr.lines[y].insert_blanks(x, n, blank);
        scr.cur.pending_wrap = false;
        self.touch(y);
    }

    pub(super) fn delete_chars(&mut self, n: usize) {
        let scr = self.scr();
        let (x, y) = (scr.cur.x, scr.cur.y);
        let blank = scr.blank();
        scr.lines[y].delete_cells(x, n, blank);
        scr.cur.pending_wrap = false;
        self.touch(y);
    }

    /// ECH: blank `n` cells from the cursor without moving anything.
    pub(super) fn erase_chars(&mut self, n: usize) {
        let scr = self.scr();
        let (x, y) = (scr.cur.x, scr.cur.y);
        let blank = scr.blank();
        let end = x.saturating_add(n);
        scr.lines[y].erase(x, end, blank);
        scr.cur.pending_wrap = false;
        self.touch(y);
    }

    /// REP: print the last character `n` more times. A screen's worth is
    /// the most that can show, so more is not done.
    pub(super) fn repeat_last(&mut self, n: usize) {
        let Some(c) = self.last_char else { return };
        let w = width(c);
        if w == 0 {
            return;
        }
        let n = n.min(MAX_REPEAT).min(self.cols * self.rows);
        for _ in 0..n {
            self.put_glyph(c, w);
        }
    }
}

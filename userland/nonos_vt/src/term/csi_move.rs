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

//! Cursor movement. Every move cancels a pending wrap. Relative moves stop
//! at the scroll region's edges when they start inside it, and absolute
//! rows count from the region when origin mode is on.

use super::state::Term;

impl Term {
    pub(super) fn moved(&mut self, x: usize, y: usize) {
        let (cols, rows) = (self.cols, self.rows);
        let cur = &mut self.scr().cur;
        cur.x = x.min(cols - 1);
        cur.y = y.min(rows - 1);
        cur.pending_wrap = false;
    }

    pub(super) fn cursor_up(&mut self, n: usize) {
        let s = self.scr_ref();
        let floor = if s.cur.y >= s.top { s.top } else { 0 };
        let (x, y) = (s.cur.x, s.cur.y.saturating_sub(n).max(floor));
        self.moved(x, y);
    }

    pub(super) fn cursor_down(&mut self, n: usize) {
        let s = self.scr_ref();
        let ceil = if s.cur.y <= s.bot { s.bot } else { self.rows - 1 };
        let (x, y) = (s.cur.x, s.cur.y.saturating_add(n).min(ceil));
        self.moved(x, y);
    }

    pub(super) fn cursor_forward(&mut self, n: usize) {
        let s = self.scr_ref();
        let (x, y) = (s.cur.x.saturating_add(n), s.cur.y);
        self.moved(x, y);
    }

    pub(super) fn cursor_back(&mut self, n: usize) {
        let s = self.scr_ref();
        let (x, y) = (s.cur.x.saturating_sub(n), s.cur.y);
        self.moved(x, y);
    }

    /// CNL and CPL: down or up `n` lines, to the first column.
    pub(super) fn cursor_line(&mut self, n: usize, down: bool) {
        if down {
            self.cursor_down(n);
        } else {
            self.cursor_up(n);
        }
        self.carriage_return();
    }
}

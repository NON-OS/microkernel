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

//! Scrolling the region, and history for the lines that leave the top.

use super::state::Term;

impl Term {
    /// Move the region's lines up `n`, blank lines entering at its foot. On
    /// the normal screen with a full-height region, the lines leaving the
    /// top go to scrollback; anywhere else they are gone, which is what a
    /// status line pinned outside the region needs.
    pub(super) fn scroll_up(&mut self, n: usize) {
        let to_history = !self.alt_active && self.scr_ref().top == 0;
        let (top, bot) = (self.scr_ref().top, self.scr_ref().bot);
        let n = n.min(bot + 1 - top);
        if n == 0 {
            return;
        }
        if to_history {
            for i in 0..n {
                let line = self.primary.lines[top + i].clone();
                self.push_history(line);
            }
        }
        let scr = self.scr();
        let blank = scr.blank();
        scr.lines[top..=bot].rotate_left(n);
        for line in &mut scr.lines[bot + 1 - n..=bot] {
            line.clear(blank);
        }
        for y in top..=bot {
            self.touch(y);
        }
    }

    /// Move the region's lines down `n`, blank lines entering at its top.
    pub(super) fn scroll_down(&mut self, n: usize) {
        let (top, bot) = (self.scr_ref().top, self.scr_ref().bot);
        let n = n.min(bot + 1 - top);
        if n == 0 {
            return;
        }
        let scr = self.scr();
        let blank = scr.blank();
        scr.lines[top..=bot].rotate_right(n);
        for line in &mut scr.lines[top..top + n] {
            line.clear(blank);
        }
        for y in top..=bot {
            self.touch(y);
        }
    }
}

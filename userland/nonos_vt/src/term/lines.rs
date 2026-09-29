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

//! Inserting and deleting whole lines inside the scroll region.

use super::state::Term;

impl Term {
    /// IL: open `n` blank lines at the cursor's row, pushing the rest of the
    /// region down and off its foot. Outside the region it does nothing.
    pub(super) fn insert_lines(&mut self, n: usize) {
        let scr = self.scr();
        let (y, top, bot) = (scr.cur.y, scr.top, scr.bot);
        if y < top || y > bot {
            return;
        }
        let n = n.min(bot + 1 - y);
        let blank = scr.blank();
        scr.lines[y..=bot].rotate_right(n);
        for line in &mut scr.lines[y..y + n] {
            line.clear(blank);
        }
        scr.cur.x = 0;
        scr.cur.pending_wrap = false;
        for r in y..=bot {
            self.touch(r);
        }
    }

    /// DL: remove `n` lines at the cursor's row, pulling the rest of the
    /// region up and blanking its foot.
    pub(super) fn delete_lines(&mut self, n: usize) {
        let scr = self.scr();
        let (y, top, bot) = (scr.cur.y, scr.top, scr.bot);
        if y < top || y > bot {
            return;
        }
        let n = n.min(bot + 1 - y);
        let blank = scr.blank();
        scr.lines[y..=bot].rotate_left(n);
        for line in &mut scr.lines[bot + 1 - n..=bot] {
            line.clear(blank);
        }
        scr.cur.x = 0;
        scr.cur.pending_wrap = false;
        for r in y..=bot {
            self.touch(r);
        }
    }
}

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

//! Moving down or up a line, scrolling the region at its edge.

use super::state::Term;

impl Term {
    /// Down a line, scrolling the region when the cursor is at its foot.
    pub(super) fn index(&mut self) {
        let rows = self.rows;
        let scr = self.scr();
        scr.cur.pending_wrap = false;
        if scr.cur.y == scr.bot {
            self.scroll_up(1);
        } else if scr.cur.y + 1 < rows {
            scr.cur.y += 1;
        }
    }

    /// Up a line, scrolling the region down when the cursor is at its top.
    pub(super) fn reverse_index(&mut self) {
        let scr = self.scr();
        scr.cur.pending_wrap = false;
        if scr.cur.y == scr.top {
            self.scroll_down(1);
        } else if scr.cur.y > 0 {
            scr.cur.y -= 1;
        }
    }

    /// Carry on at the start of the next line, marking this one as running
    /// on so copying and re-wrapping join them.
    pub(super) fn wrap_to_next_line(&mut self) {
        let y = self.scr_ref().cur.y;
        self.scr().lines[y].wrapped = true;
        self.scr().cur.x = 0;
        self.index();
    }
}

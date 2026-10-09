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

//! Absolute cursor positions and the scroll region, counted from the
//! region in origin mode.

use super::state::Term;

impl Term {
    /// CHA and HPA, 1-based.
    pub(super) fn cursor_column(&mut self, col: usize) {
        let y = self.scr_ref().cur.y;
        self.moved(col.saturating_sub(1), y);
    }

    /// VPA, 1-based.
    pub(super) fn cursor_row(&mut self, row: usize) {
        let x = self.scr_ref().cur.x;
        let y = self.origin_row(row);
        self.moved(x, y);
    }

    /// CUP, 1-based.
    pub(super) fn cursor_to(&mut self, row: usize, col: usize) {
        let y = self.origin_row(row);
        self.moved(col.saturating_sub(1), y);
    }

    pub(super) fn origin_row(&self, row: usize) -> usize {
        let s = self.scr_ref();
        let r = row.saturating_sub(1);
        if self.modes.origin {
            s.top.saturating_add(r).min(s.bot)
        } else {
            r
        }
    }

    /// DECSTBM, 1-based and inclusive; 0 for the bottom means the last row.
    /// An impossible region is ignored, as xterm ignores it.
    pub(super) fn set_region(&mut self, top: usize, bot: usize) {
        let rows = self.rows;
        let bot = if bot == 0 { rows } else { bot.min(rows) };
        let top = top.max(1);
        if top >= bot {
            return;
        }
        let scr = self.scr();
        scr.top = top - 1;
        scr.bot = bot - 1;
        let home = if self.modes.origin { top - 1 } else { 0 };
        self.moved(0, home);
    }
}

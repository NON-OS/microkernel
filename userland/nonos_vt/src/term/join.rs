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

//! Drawing a mark over the character before it rather than in a cell of
//! its own.

use super::state::Term;
use crate::cell::Cell;

impl Term {
    /// The cell the last character went into.
    pub(super) fn prev_x(&self) -> Option<usize> {
        let cur = self.scr_ref().cur;
        let x = if cur.pending_wrap { cur.x } else { cur.x.checked_sub(1)? };
        let line = &self.scr_ref().lines[cur.y];
        if line.cell(x).is_tail() {
            x.checked_sub(1)
        } else {
            Some(x)
        }
    }

    pub(super) fn prev_cell(&self) -> Option<Cell> {
        let x = self.prev_x()?;
        let y = self.scr_ref().cur.y;
        Some(self.scr_ref().lines[y].cell(x))
    }

    pub(super) fn attach_mark(&mut self, c: char) -> bool {
        let Some(x) = self.prev_x() else { return false };
        let y = self.scr_ref().cur.y;
        let ok = self.scr().lines[y].add_mark(x, c);
        self.touch(y);
        ok
    }
}

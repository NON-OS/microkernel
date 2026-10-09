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

//! Taking the normal screen and its history apart into physical lines.

use alloc::vec::Vec;

use crate::line::Line;
use crate::term::Term;

pub(super) struct Gathered {
    pub(super) phys: Vec<Line>,
    /// The cursor's physical line, and its offset into that line; one past
    /// the last column when it was waiting to wrap.
    pub(super) cur_phys: usize,
    pub(super) cur_col: usize,
    /// Where the old screen starts among the physical lines.
    pub(super) old_top: usize,
}

impl Term {
    /// History then the screen down to its last used row, which the cursor
    /// counts as using. Blank rows below are dropped.
    pub(super) fn gather(&mut self) -> Gathered {
        let cur = self.primary.cur;
        let old_top = self.scrollback.len();
        let mut phys: Vec<Line> = self.scrollback.drain(..).collect();
        let used = self
            .primary
            .lines
            .iter()
            .rposition(|l| l.content_len() > 0 || l.wrapped)
            .map_or(0, |i| i + 1)
            .max(cur.y + 1);
        phys.extend(self.primary.lines.drain(..).take(used));
        Gathered {
            phys,
            cur_phys: old_top + cur.y,
            cur_col: cur.x + cur.pending_wrap as usize,
            old_top,
        }
    }
}

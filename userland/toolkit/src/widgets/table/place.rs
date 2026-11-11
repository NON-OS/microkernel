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

//! Where a table draws each cell and header, from `parts`, for the marks an
//! app draws over them.

use super::geometry::{ink, Span};
use super::Table;
use crate::decorations::Rect;

impl Table<'_> {
    /// The cell in column `col` of the row with whole-list index `row`, as
    /// `draw` paints it and `hit` names it, in `draw`'s coordinates. `None`
    /// when the column is past the last, or the row is not among the given
    /// rows that fit whole in `at`. A column with no width has an empty rect.
    /// A disabled table has the same rects, though `hit` names none of them.
    pub fn cell_rect(&self, at: Rect, row: usize, col: usize) -> Option<Rect> {
        let p = self.parts(at);
        let band = *p.rows.get(row.checked_sub(self.first)?)?;
        p.cols.get(col).map(|s| across(s, band))
    }

    /// The header of column `col` as `draw` paints it and `hit` names it, or
    /// `None` when the column is past the last.
    pub fn head_rect(&self, at: Rect, col: usize) -> Option<Rect> {
        let p = self.parts(at);
        p.cols.get(col).map(|s| across(s, p.head))
    }

    /// The part of `cell`, a rect from `cell_rect` or `head_rect`, that the
    /// table lays its text out in: `SPACE.md` in from each side, or half its
    /// width when narrower, and the cell's full height.
    pub fn text_rect(cell: Rect) -> Rect {
        let (x, w) = ink(cell.x, cell.w);
        Rect { x, w, ..cell }
    }
}

/// The part of `band` that column `s` spans.
pub(super) fn across(s: &Span, band: Rect) -> Rect {
    Rect { x: s.x, w: s.w, ..band }
}

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

//! Moving a line's cells: inserting, deleting, resizing and trimming.

use super::Line;
use crate::cell::Cell;

impl Line {
    /// Open `n` blank cells at `x`, pushing the rest right and off the end.
    pub fn insert_blanks(&mut self, x: usize, n: usize, fill: Cell) {
        let len = self.cells.len();
        if x >= len || n == 0 {
            return;
        }
        let n = n.min(len - x);
        self.split_wide_at(x);
        self.cells[x..].rotate_right(n);
        for c in &mut self.cells[x..x + n] {
            *c = fill;
        }
        self.drop_cut_head();
    }

    /// Remove `n` cells at `x`, pulling the rest left and blanking the end.
    pub fn delete_cells(&mut self, x: usize, n: usize, fill: Cell) {
        let len = self.cells.len();
        if x >= len || n == 0 {
            return;
        }
        let n = n.min(len - x);
        self.split_wide_at(x);
        self.split_wide_at(x + n - 1);
        self.cells[x..].rotate_left(n);
        for c in &mut self.cells[len - n..] {
            *c = fill;
        }
    }

    /// A wide character pushed half off the right edge loses its head too.
    fn drop_cut_head(&mut self) {
        if let Some(last) = self.cells.last_mut() {
            if last.is_wide() {
                let bg = last.bg;
                *last = Cell { bg, ..Cell::BLANK };
            }
        }
    }

    /// Change the width to `cols`, padding with `fill`.
    pub fn resize(&mut self, cols: usize, fill: Cell) {
        self.cells.resize(cols, fill);
        self.drop_cut_head();
    }
}

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

//! Writes into a line that keep two-column characters whole.

use super::Line;
use crate::cell::Cell;

impl Line {
    /// Blank the other half of any wide character that has a cell at `x`
    /// about to be overwritten.
    pub(super) fn split_wide_at(&mut self, x: usize) {
        let Some(c) = self.cells.get(x).copied() else { return };
        if c.is_tail() && x > 0 {
            let bg = self.cells[x - 1].bg;
            self.cells[x - 1] = Cell { bg, ..Cell::BLANK };
        }
        if c.is_wide() {
            if let Some(t) = self.cells.get_mut(x + 1) {
                let bg = t.bg;
                *t = Cell { bg, ..Cell::BLANK };
            }
        }
    }

    pub fn put(&mut self, x: usize, cell: Cell) {
        if x >= self.cells.len() {
            return;
        }
        self.split_wide_at(x);
        self.cells[x] = cell;
    }

    /// Write a two-column character: its head at `x`, its tail after.
    pub fn put_pair(&mut self, x: usize, head: Cell, tail: Cell) {
        if x + 1 >= self.cells.len() {
            return;
        }
        self.split_wide_at(x);
        self.split_wide_at(x + 1);
        self.cells[x] = head;
        self.cells[x + 1] = tail;
    }

    /// Blank `[from, to)`.
    pub fn erase(&mut self, from: usize, to: usize, fill: Cell) {
        let to = to.min(self.cells.len());
        if from >= to {
            return;
        }
        self.split_wide_at(from);
        self.split_wide_at(to - 1);
        for c in &mut self.cells[from..to] {
            *c = fill;
        }
    }
}

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

//! The line itself.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::cell::Cell;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub cells: Vec<Cell>,
    /// The text ran on to the next line rather than ending here, so a copy or
    /// a re-wrap joins the two.
    pub wrapped: bool,
    pub(super) marks: Vec<String>,
}

impl Line {
    pub fn new(cols: usize, fill: Cell) -> Line {
        Line { cells: vec![fill; cols], wrapped: false, marks: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cells.is_empty()
    }

    pub fn cell(&self, x: usize) -> Cell {
        self.cells.get(x).copied().unwrap_or(Cell::BLANK)
    }

    pub fn clear(&mut self, fill: Cell) {
        for c in self.cells.iter_mut() {
            *c = fill;
        }
        self.marks.clear();
        self.wrapped = false;
    }

    /// Append `cell`, carrying its marks over from `src`.
    pub fn push_from(&mut self, src: &Line, cell: Cell) {
        self.cells.push(Cell { mark: 0, ..cell });
        if let Some(s) = src.marks_of(&cell) {
            let x = self.cells.len() - 1;
            for ch in s.chars() {
                self.add_mark(x, ch);
            }
        }
    }
}

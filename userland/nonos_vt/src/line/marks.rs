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

//! Combining marks, held beside the cells they are drawn over.

use alloc::string::String;

use super::Line;
use crate::cell::Cell;
use crate::limits::MAX_MARKS_PER_LINE;

impl Line {
    /// The combining marks drawn over `cell`, if it has any.
    pub fn marks_of(&self, cell: &Cell) -> Option<&str> {
        let i = (cell.mark as usize).checked_sub(1)?;
        self.marks.get(i).map(|s| s.as_str())
    }

    /// Put a combining mark over the cell at `x`. Refused, and the mark
    /// dropped, once the line holds as many as it may.
    pub fn add_mark(&mut self, x: usize, c: char) -> bool {
        let Some(cell) = self.cells.get(x).copied() else { return false };
        if let Some(i) = (cell.mark as usize).checked_sub(1) {
            if let Some(s) = self.marks.get_mut(i) {
                let room = s.len() < 32;
                if room {
                    s.push(c);
                }
                return room;
            }
        }
        if self.marks.len() >= MAX_MARKS_PER_LINE {
            self.compact_marks();
            if self.marks.len() >= MAX_MARKS_PER_LINE {
                return false;
            }
        }
        self.marks.push(String::from(c));
        self.cells[x].mark = self.marks.len() as u16;
        true
    }

    /// Drop marks no cell points at any more, renumbering the rest.
    fn compact_marks(&mut self) {
        let old = core::mem::take(&mut self.marks);
        for cell in self.cells.iter_mut() {
            let Some(i) = (cell.mark as usize).checked_sub(1) else { continue };
            cell.mark = match old.get(i) {
                Some(s) => {
                    self.marks.push(s.clone());
                    self.marks.len() as u16
                }
                None => 0,
            };
        }
    }
}

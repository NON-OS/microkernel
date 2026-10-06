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

//! Reading a pointer position back as a cell.

use nonos_vt::{Pos, Term};

use super::cells::CellGeom;
use crate::paint::Rows;

impl CellGeom {
    /// The cell under `(px, py)`, clamped to the screen, and the row it is.
    pub fn cell_at(&self, vt: &Term, px: i32, py: i32) -> (usize, usize) {
        let col = ((px - self.x as i32).max(0) as u32 / self.adv.max(1)) as usize;
        let row = ((py - self.y as i32).max(0) as u32 / self.lh.max(1)) as usize;
        (col.min(vt.cols() - 1), row.min(self.body_rows(vt).saturating_sub(1)))
    }

    pub fn pos_at(&self, vt: &Term, px: i32, py: i32) -> Pos {
        let (col, row) = self.cell_at(vt, px, py);
        let rows = self.rows(vt);
        let line = match rows {
            Rows::Screen => vt.abs_of_row(row),
            Rows::Shell { .. } => rows.first(vt) + row as u64,
        };
        Pos { line, col }
    }

    /*
     * Over the text or the margin around it: a press a few pixels short of
     * the first column is aimed at it, and cell_at clamps it there.
     */
    pub fn inside(&self, vt: &Term, px: i32, py: i32) -> bool {
        let w = vt.cols() as u32 * self.adv;
        let h = self.body_rows(vt) as u32 * self.lh;
        let left = self.x as i32 - self.pad as i32;
        px >= left
            && py >= self.y as i32
            && px < (self.x + w + self.pad) as i32
            && py < (self.y + h) as i32
    }
}

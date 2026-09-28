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

//! Where the body's cells were drawn in the last paint, so a pointer event
//! can be read back as a cell and a line.

use nonos_vt::{Pos, Term};

use crate::paint::Rows;

#[derive(Clone, Copy)]
pub struct CellGeom {
    pub x: u32,
    pub y: u32,
    pub adv: u32,
    pub lh: u32,
    /// Rows of the body a shell drawing uses; the screen owns all rows.
    pub shell_rows: usize,
    pub owned: bool,
}

impl CellGeom {
    pub fn rows(&self, vt: &Term) -> Rows {
        if self.owned {
            Rows::Screen
        } else {
            Rows::Shell { rows: self.shell_rows, back: vt.view_offset() }
        }
    }

    /// The cell under `(px, py)`, clamped to the screen, and the row it is.
    pub fn cell_at(&self, vt: &Term, px: i32, py: i32) -> (usize, usize) {
        let col = ((px - self.x as i32).max(0) as u32 / self.adv.max(1)) as usize;
        let row = ((py - self.y as i32).max(0) as u32 / self.lh.max(1)) as usize;
        let max_row = if self.owned { vt.rows() } else { self.shell_rows };
        (col.min(vt.cols() - 1), row.min(max_row.saturating_sub(1)))
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

    pub fn inside(&self, vt: &Term, px: i32, py: i32) -> bool {
        let w = vt.cols() as u32 * self.adv;
        let rows = if self.owned { vt.rows() } else { self.shell_rows };
        let h = rows as u32 * self.lh;
        px >= self.x as i32
            && py >= self.y as i32
            && px < (self.x + w) as i32
            && py < (self.y + h) as i32
    }
}

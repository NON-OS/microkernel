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

use nonos_vt::Term;

use crate::paint::Rows;

#[derive(Clone, Copy)]
pub struct CellGeom {
    pub x: u32,
    pub y: u32,
    pub adv: u32,
    pub lh: u32,
    /// The margin drawn around the text, which a press may land in.
    pub pad: u32,
    /// Rows of the body a shell drawing uses; the screen owns all rows.
    pub shell_rows: usize,
    pub owned: bool,
}

impl CellGeom {
    pub(super) fn body_rows(&self, vt: &Term) -> usize {
        if self.owned {
            vt.rows()
        } else {
            self.shell_rows
        }
    }

    pub fn rows(&self, vt: &Term) -> Rows {
        if self.owned {
            Rows::Screen
        } else {
            Rows::Shell { rows: self.shell_rows, back: vt.view_offset() }
        }
    }
}

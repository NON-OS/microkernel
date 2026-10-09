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

//! What a frame needs besides the rows: the cursor, which rows changed,
//! and each cell's final colours.

use super::state::Term;
use super::types::CursorView;
use crate::cell::{attr, Cell};
use crate::color::{dim, Palette};
use alloc::vec::Vec;

impl Term {
    pub fn cursor(&self) -> CursorView {
        let s = self.scr_ref();
        CursorView {
            x: s.cur.x,
            y: s.cur.y,
            visible: self.modes.cursor_visible && self.view == 0,
            shape: self.cursor_shape,
            blink: self.modes.cursor_blink,
        }
    }

    /// Which rows changed since the last call, then forgets.
    pub fn take_dirty(&mut self) -> Vec<bool> {
        let fresh = alloc::vec![false; self.rows];
        core::mem::replace(&mut self.dirty, fresh)
    }

    pub fn palette(&self) -> &Palette {
        &self.palette
    }

    /// The colours a cell is drawn in, after bold brightening, dim, inverse,
    /// hidden and reverse video.
    pub fn cell_colors(&self, cell: &Cell) -> (u32, u32) {
        let bold = cell.attr & attr::BOLD != 0;
        let mut fg = self.palette.fg_of(cell.fg, bold);
        let mut bg = self.palette.bg_of(cell.bg);
        if cell.attr & attr::DIM != 0 {
            fg = dim(fg);
        }
        if (cell.attr & attr::INVERSE != 0) != self.modes.reverse_video {
            core::mem::swap(&mut fg, &mut bg);
        }
        if cell.attr & attr::HIDDEN != 0 {
            fg = bg;
        }
        (fg, bg)
    }
}

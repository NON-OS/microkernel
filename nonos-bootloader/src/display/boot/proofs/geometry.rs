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

//! Where the panel sits: in the splash's right column, over the log, as
//! numbered rows on thin rules like the menu's.

use super::rows::ROWS;
use crate::display::boot::layout::splash;
use crate::display::ink::{label_width, metrics, Style};

#[derive(Clone, Copy)]
pub struct Frame {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub u: u32,
    pub head_h: u32,
    pub row_h: u32,
    /// Columns from the left edge: name, and the detail under it.
    pub label_x: u32,
    pub detail_x: u32,
}

/// The panel's head and row heights for spacing unit `u`.
fn heights(u: u32) -> (u32, u32) {
    let (mono, body) = (metrics(Style::Mono), metrics(Style::Body));
    (mono.line + 4 * u, body.line + mono.line + 3 * u)
}

/// The panel's height, which the splash leaves room for.
pub fn panel_height(u: u32) -> u32 {
    let (head_h, row_h) = heights(u);
    head_h + row_h * ROWS as u32 + metrics(Style::Mono).line * 3
}

pub fn frame() -> Frame {
    let s = splash();
    let u = s.u;
    let (head_h, row_h) = heights(u);
    let label_x = label_width(b"00") + 3 * u;
    Frame {
        x: s.col_x,
        y: s.panel_y,
        w: s.col_w,
        h: panel_height(u),
        u,
        head_h,
        row_h,
        label_x,
        detail_x: label_x,
    }
}

impl Frame {
    /// The top of row `i`.
    pub fn row_y(&self, i: usize) -> u32 {
        self.y + self.head_h + self.row_h * i as u32
    }
}

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
//! Where the switcher's strip and its cells sit: one row, centred a little
//! above the middle of the screen, as many cells as the width holds, the
//! row's window following the run past the last that fits.

use crate::render::layout::Rect;
use crate::render::ui_font::px;

/// A cell: the tile, the gap and the name under it, in logical pixels.
pub const CELL_W: u32 = 120;
pub const CELL_H: u32 = 116;
pub const TILE: u32 = 64;
/// The panel's edge around the cells.
pub const PAD: u32 = 16;

/// How many cells show of `n` on a screen `w` wide.
pub fn fits(w: u32, n: usize) -> usize {
    let room = w.saturating_sub(px(2 * PAD + 2 * 24)) / px(CELL_W).max(1);
    n.min(room.max(1) as usize)
}

/// The first window shown, keeping the run's window `at` in the row: the
/// row starts at the first window until the run passes its last cell.
pub fn first(at: usize, shown: usize) -> usize {
    at.saturating_sub(shown.saturating_sub(1))
}

/// The panel around `shown` cells on a `w` by `h` screen.
pub fn panel(w: u32, h: u32, shown: usize) -> Rect {
    let (pw, ph) = (px(2 * PAD + shown as u32 * CELL_W).min(w), px(2 * PAD + CELL_H).min(h));
    let y = (h.saturating_sub(ph) * 2 / 5).max(px(48)).min(h.saturating_sub(ph));
    Rect { x: w.saturating_sub(pw) / 2, y, width: pw, height: ph }
}

/// The `i`th cell shown (0 is the row's first).
pub fn cell(p: Rect, i: usize) -> Rect {
    let x = p.x + px(PAD + i as u32 * CELL_W);
    Rect { x, y: p.y + px(PAD), width: px(CELL_W), height: px(CELL_H) }
}

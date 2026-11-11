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

extern crate alloc;

use alloc::vec::Vec;

use super::home_cats::CATS;
use super::home_geom::{cat_h, cat_row_y};
use super::layout::{metrics, px};

/// The gap between two cards in the row. Every card takes an equal share of what
/// is left once the gaps are reserved, so the row fills the content width at any
/// window size instead of stranding a fixed span.
pub const CARD_GAP: u32 = 10;

/// One laid-out category card. The painter draws exactly these rectangles and
/// `home_hit` tests exactly these rectangles, so a click can never land on a
/// card other than the one it appears to be on.
pub struct CatCell {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub idx: usize,
}

/// The row's one layout pass.
pub fn cat_cells(win_w: u32) -> Vec<CatCell> {
    let n = CATS.len() as u32;
    let (m, gap) = (metrics(), px(CARD_GAP));
    let total = win_w.saturating_sub(m.sidebar_w + m.pad_x * 2);
    let span = total.saturating_sub(gap * (n - 1)) / n;
    let (y, h) = (cat_row_y(), cat_h());
    (0..CATS.len())
        .map(|i| CatCell {
            x: m.sidebar_w + m.pad_x + (span + gap) * i as u32,
            y,
            w: span,
            h,
            idx: i,
        })
        .collect()
}

/// Which card `(x, y)` is inside, tested against the laid-out cells themselves.
pub fn cat_at(win_w: u32, x: u32, y: u32) -> Option<usize> {
    cat_cells(win_w)
        .into_iter()
        .find(|c| x >= c.x && x < c.x + c.w && y >= c.y && y < c.y + c.h)
        .map(|c| c.idx)
}

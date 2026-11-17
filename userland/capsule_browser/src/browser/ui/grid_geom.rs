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

use super::list_geom::{ListGeom, BOTTOM};
use super::rect::Rect;
use super::settings_cells::px;

const GAP: u32 = 20;
const CARD_H: u32 = 196;
const MIN_W: u32 = 200;
const CLOSE: u32 = 28;
const CLOSE_INSET: u32 = 10;
const CHIP_INSET: u32 = 24;

impl ListGeom {
    /// Four columns of cards, or three, two or one as the page narrows.
    pub fn cols(&self) -> i32 {
        let (min, gap) = (px(MIN_W), px(GAP));
        (1..=4).rev().find(|&c| c * min + (c - 1) * gap <= self.frame.w).unwrap_or(1)
    }

    pub fn grid_rect(&self, i: usize, cols: i32) -> Rect {
        let (cols, gap, card_h) = (cols.max(1), px(GAP), px(CARD_H));
        let cw = ((self.frame.w - (cols - 1) * gap) / cols).max(0);
        let (col, row) = (i as i32 % cols, i as i32 / cols);
        let y = self.band - self.scroll + row * (card_h + gap);
        Rect::new(self.frame.x + col * (cw + gap), y, cw, card_h)
    }

    /// The height of the whole page when its grid holds `n` cards.
    pub fn grid_h(&self, n: usize, cols: i32) -> i32 {
        let rows = (n as i32 + cols.max(1) - 1) / cols.max(1);
        self.band + rows * px(CARD_H) + (rows - 1).max(0) * px(GAP) + px(BOTTOM)
    }
}

/// The close button in the top right corner of a grid card.
pub fn close_rect(card: Rect) -> Rect {
    let (side, inset) = (px(CLOSE), px(CLOSE_INSET));
    Rect::new(card.right() - side - inset, card.y + inset, side, side)
}

/// The group chip at the bottom left of a grid card, `w` by `h` at most.
pub fn chip_rect(card: Rect, w: i32, h: i32) -> Rect {
    let inset = px(CHIP_INSET);
    let w = w.min(card.w - 2 * inset).max(0);
    Rect::new(card.x + inset, card.bottom() - inset - h, w, h)
}

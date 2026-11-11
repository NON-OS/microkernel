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

//! One source of truth for ribbon geometry: the painter records the cells this
//! builds and the hit-test reads them back, so a click lands on the cell that
//! was drawn under it even though the pill labels change width with the caret.

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::ttf::line_height;

use super::items::RibbonItem;
use crate::editor::layout::{chrome_px, ribbon_h, tabbar_h, titlebar_h};

pub(in crate::editor) struct RibbonCell {
    pub x0: u32,
    pub x1: u32,
    pub item: RibbonItem,
}

pub(super) struct Geom {
    pub cy: u32,
    pub ch: u32,
    pub ty: i32,
}

/// The ribbon's own lengths, in drawing pixels.
pub(super) struct Lengths {
    pub bar_pad: u32,
    pub pill_pad: u32,
    pub chevron_w: u32,
    pub chevron: u32,
    pub gap: u32,
    pub sep_gap: u32,
    pub square_w: u32,
}

const BAR_PAD: u32 = 12;
const PILL_PAD: u32 = 10;
const CHEVRON_W: u32 = 16;
const CHEVRON: u32 = 12;
const GAP: u32 = 6;
const SEP_GAP: u32 = 15;
const SQUARE_W: u32 = 30;
const CELL_MIN: u32 = 14;
const CELL_PAD: u32 = 6;

/// Every ribbon length at the scale in force, read once per paint or hit.
pub(super) fn lengths() -> Lengths {
    let s = scale();
    Lengths {
        bar_pad: s.px(BAR_PAD),
        pill_pad: s.px(PILL_PAD),
        chevron_w: s.px(CHEVRON_W),
        chevron: s.px(CHEVRON),
        gap: s.px(GAP),
        sep_gap: s.px(SEP_GAP),
        square_w: s.px(SQUARE_W),
    }
}

pub(super) fn band_top() -> u32 {
    titlebar_h() + tabbar_h()
}

pub(super) fn cell_h() -> u32 {
    let (s, lh) = (scale(), line_height(chrome_px()).max(0) as u32);
    lh.max(s.px(CELL_MIN)) + s.px(CELL_PAD)
}

pub(super) fn cell_top() -> u32 {
    band_top() + ribbon_h().saturating_sub(cell_h()) / 2
}

pub(super) fn text_top(box_h: u32) -> u32 {
    let lh = line_height(chrome_px()).max(1) as u32;
    box_h.saturating_sub(lh) / 2
}

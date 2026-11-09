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

use nonos_toolkit::design::scale::scale;
use nonos_toolkit::design::size::PILL_H;
use nonos_toolkit::widgets::SectionHeader;

pub use super::list_parts::{max_scroll, middle_pills};
use super::rect::Rect;
use super::settings_cells::px;

const BAND: u32 = 168;
pub(super) const BOTTOM: u32 = 32;
const ROW_H: u32 = 56;
const SIDE: u32 = 40;
const MAX_W: u32 = 960;
const TITLE_Y: u32 = 28;
const PILLS_Y: u32 = 114;
pub(super) const PILL_GAP: u32 = 8;
const CARD_PAD: u32 = 6;

/// Where an internal page's header, filter pills and list sit, in
/// content-local pixels at the display scale. The header band stays put;
/// the list under it moves up by `scroll`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ListGeom {
    pub frame: Rect,
    pub head: Rect,
    pub pills_y: i32,
    pub pill_h: i32,
    pub band: i32,
    pub row_h: i32,
    pub scroll: i32,
}

impl ListGeom {
    pub fn new(w: i32, h: i32, scroll: i32) -> Self {
        let (w, h) = (w.max(0), h.max(0));
        let fw = px(MAX_W).min((w - 2 * px(SIDE)).max(0));
        let frame = Rect::new((w - fw) / 2, 0, fw, h);
        let head_h = SectionHeader::new("").note("").size().1 as i32;
        let head = Rect::new(frame.x, px(TITLE_Y), fw, head_h);
        let (pills_y, pill_h) = (px(PILLS_Y), scale().px(PILL_H) as i32);
        let (band, row_h, scroll) = (px(BAND), px(ROW_H), scroll.max(0));
        ListGeom { frame, head, pills_y, pill_h, band, row_h, scroll }
    }

    /// Pill `i` of a row whose pills are `widths` wide, left to right.
    pub fn pill(&self, widths: &[i32], i: usize) -> Rect {
        let x = widths.iter().take(i).fold(self.frame.x, |x, w| x + w + px(PILL_GAP));
        Rect::new(x, self.pills_y, widths.get(i).copied().unwrap_or(0), self.pill_h)
    }

    /// The card holding `n` rows.
    pub fn card(&self, n: usize) -> Rect {
        let h = n as i32 * self.row_h + 2 * px(CARD_PAD);
        Rect::new(self.frame.x, self.band - self.scroll, self.frame.w, h)
    }

    pub fn row_rect(&self, i: usize) -> Rect {
        let c = self.card(0);
        Rect::new(c.x, c.y + px(CARD_PAD) + i as i32 * self.row_h, c.w, self.row_h)
    }

    /// The height of the whole page when its list has `n` rows.
    pub fn list_h(&self, n: usize) -> i32 {
        self.band + self.card(n).h + px(BOTTOM)
    }
}

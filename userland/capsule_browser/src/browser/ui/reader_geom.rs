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

use nonos_toolkit::design::size::{CARD_HEAD_H, CONTROL_H, ROW_H, SPACE};

use super::rect::Rect;
use super::settings_cells::px;

pub const PANEL_W: i32 = 300;
pub const MIN_W: i32 = 200;
pub const MARGIN: i32 = 16;
const LABEL: u32 = 22;
const LABEL_GAP: u32 = 8;
const SECTION: u32 = 20;
const SWATCH_H: u32 = 56;
const SEG_GAP: u32 = 8;

/// Every part of the reader settings card, in content-area coordinates at
/// the display scale. The painter lays its controls out from this.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReaderGeom {
    pub panel: Rect,
    pub close: Rect,
    pub labels: [Rect; 3],
    pub swatches: [Rect; 3],
    pub minus: Rect,
    pub plus: Rect,
    pub dots: Rect,
    pub widths: [Rect; 3],
    pub time: Rect,
}

impl ReaderGeom {
    /// True when the card is wide enough to be drawn.
    pub fn drawn(&self) -> bool {
        self.panel.w >= px(MIN_W as u32)
    }
}

/// The card on the right of a `w` wide content area: its head with the
/// close button, the three labelled sections, then the reading-time row
/// across the card's full width.
pub fn panel(w: i32) -> ReaderGeom {
    let (m, pad, ctl) = (px(MARGIN as u32), px(u32::from(SPACE.lg)), px(CONTROL_H));
    let pw = px(PANEL_W as u32).min(w - 2 * m).max(0);
    let (x, iw) = ((w - m - pw).max(0), (pw - 2 * pad).max(0));
    let ix = x + pad;
    let head = px(CARD_HEAD_H);
    let close = Rect::new(ix + iw - ctl, m + (head - ctl) / 2, ctl, ctl);
    let mut y = m + head;
    let mut label = |h: i32| {
        let r = Rect::new(ix, y, iw, px(LABEL));
        y += px(LABEL) + px(LABEL_GAP) + h + px(SECTION);
        (r, Rect::new(ix, r.bottom() + px(LABEL_GAP), iw, h))
    };
    let (l0, swatches) = label(px(SWATCH_H));
    let (l1, size) = label(ctl);
    let (l2, widths) = label(ctl);
    let minus = Rect::new(ix, size.y, ctl, ctl);
    let plus = Rect::new(size.right() - ctl, size.y, ctl, ctl);
    let dots = Rect::new(minus.right(), size.y, (plus.x - minus.right()).max(0), ctl);
    let time = Rect::new(x, y - px(SECTION / 2), pw, px(ROW_H));
    let panel = Rect::new(x, m, pw, time.bottom() + px(u32::from(SPACE.sm)) - m);
    ReaderGeom {
        panel,
        close,
        labels: [l0, l1, l2],
        swatches: swatches.thirds(px(SEG_GAP)),
        minus,
        plus,
        dots,
        widths: widths.thirds(px(SEG_GAP)),
        time,
    }
}

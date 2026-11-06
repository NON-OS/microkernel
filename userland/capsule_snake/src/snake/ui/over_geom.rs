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

use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::widgets::rect::{centre, inset};
use nonos_toolkit::widgets::Rect;

use super::metrics::{BTN_H, GAP, GAP_WIDE, MODAL_H, MODAL_W, PAD, PX_TITLE};
use super::rect::{self, at};

pub const ACTIONS: usize = 3;

pub const LABELS: [&[u8]; ACTIONS] = [b"Play Again", b"Ranks", b"Home"];

pub fn title_h() -> u32 {
    line_height(PX_TITLE).max(1) as u32
}

pub fn panel(w: u32, h: u32) -> Rect {
    let want_h = MODAL_H + BTN_H + GAP_WIDE;
    centre(rect::content(w, h), MODAL_W * 2, want_h)
}

pub fn title(w: u32, h: u32) -> Rect {
    let inner = inset(panel(w, h), PAD);
    Rect { h: title_h(), ..inner }
}

// The panel interior between the title and the action row. `over_geom_rows`
// splits it into the still and the summary.
pub fn body(w: u32, h: u32) -> Rect {
    let inner = inset(panel(w, h), PAD);
    let top = title_h() + GAP;
    let h = inner.h.saturating_sub(top + BTN_H + GAP_WIDE);
    at(inner.x, inner.y + top, inner.w, h)
}

pub fn action(w: u32, h: u32, index: usize) -> Rect {
    let inner = inset(panel(w, h), PAD);
    let band = at(inner.x, inner.y + inner.h.saturating_sub(BTN_H), inner.w, BTN_H);
    rect::column(band, index, ACTIONS, GAP)
}

pub fn action_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(ACTIONS, x, y, |i| action(w, h, i))
}

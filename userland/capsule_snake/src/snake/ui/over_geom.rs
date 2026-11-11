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

use super::metrics::metrics;
use super::rect::{self, at};

pub const ACTIONS: usize = 3;

pub const LABELS: [&[u8]; ACTIONS] = [b"Play Again", b"Ranks", b"Home"];

pub fn title_h() -> u32 {
    line_height(metrics().px_title).max(1) as u32
}

pub fn panel(w: u32, h: u32) -> Rect {
    let m = metrics();
    let want_h = m.modal_h + m.btn_h + m.gap_wide;
    centre(rect::content(w, h), m.modal_w * 2, want_h)
}

pub fn title(w: u32, h: u32) -> Rect {
    let inner = inset(panel(w, h), metrics().pad);
    Rect { h: title_h(), ..inner }
}

// The panel interior between the title and the action row. `over_geom_rows`
// splits it into the still and the summary.
pub fn body(w: u32, h: u32) -> Rect {
    let m = metrics();
    let inner = inset(panel(w, h), m.pad);
    let top = title_h() + m.gap;
    let h = inner.h.saturating_sub(top + m.btn_h + m.gap_wide);
    at(inner.x, inner.y + top, inner.w, h)
}

pub fn action(w: u32, h: u32, index: usize) -> Rect {
    let m = metrics();
    let inner = inset(panel(w, h), m.pad);
    let band = at(inner.x, inner.y + inner.h.saturating_sub(m.btn_h), inner.w, m.btn_h);
    rect::column(band, index, ACTIONS, m.gap)
}

pub fn action_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(ACTIONS, x, y, |i| action(w, h, i))
}

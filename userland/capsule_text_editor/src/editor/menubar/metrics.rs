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

//! One source of truth for menu geometry: the painter and the hit-test both
//! derive title cells and panel rows from these measured functions.

use alloc::vec::Vec;
use nonos_app_skeleton::measure_ttf;
use nonos_toolkit::ttf::line_height;
use nonos_toolkit::widgets::{Item, Rect};

use super::items::TITLES;
use crate::editor::layout::{CHROME_PX, TITLEBAR_H};
use crate::editor::menu_view::place;

pub(in crate::editor) struct TitleSpan {
    pub x0: u32,
    pub x1: u32,
}

pub(super) const BAR_X0: u32 = 8;
pub(super) const TITLE_PAD: u32 = 12;

pub(super) fn text_top(box_h: u32) -> u32 {
    let lh = line_height(CHROME_PX).max(1) as u32;
    box_h.saturating_sub(lh) / 2
}

pub(super) fn title_spans() -> Vec<TitleSpan> {
    let mut out = Vec::new();
    let mut x = BAR_X0;
    for title in TITLES.iter() {
        let w = measure_ttf(title, CHROME_PX).max(0) as u32 + TITLE_PAD * 2;
        out.push(TitleSpan { x0: x, x1: x + w });
        x += w;
    }
    out
}

/// Where the drop-down of `items` for title `open` is drawn: right under its
/// title, kept inside `bounds`.
pub(super) fn panel_at(spans: &[TitleSpan], open: usize, items: &[Item], bounds: Rect) -> Rect {
    let (x0, x1) = spans.get(open).map_or((BAR_X0, BAR_X0), |s| (s.x0, s.x1));
    place(items, Rect { x: x0, y: 0, w: x1 - x0, h: TITLEBAR_H }, bounds)
}

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
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::ttf::line_height;
use nonos_toolkit::widgets::{Item, Rect};

use super::items::TITLES;
use crate::editor::layout::{chrome_px, titlebar_h};
use crate::editor::menu_view::place;

pub(in crate::editor) struct TitleSpan {
    pub x0: u32,
    pub x1: u32,
}

const BAR_X0: u32 = 8;
const TITLE_PAD: u32 = 12;

/// The first title's left edge, in drawing pixels.
pub(super) fn bar_x0() -> u32 {
    scale().px(BAR_X0)
}

/// The gap either side of a title's label, in drawing pixels.
pub(super) fn title_pad() -> u32 {
    scale().px(TITLE_PAD)
}

pub(super) fn text_top(box_h: u32) -> u32 {
    let lh = line_height(chrome_px()).max(1) as u32;
    box_h.saturating_sub(lh) / 2
}

pub(super) fn title_spans() -> Vec<TitleSpan> {
    let mut out = Vec::new();
    let (mut x, pad, px) = (bar_x0(), title_pad(), chrome_px());
    for title in TITLES.iter() {
        let w = measure_ttf(title, px).max(0) as u32 + pad * 2;
        out.push(TitleSpan { x0: x, x1: x + w });
        x += w;
    }
    out
}

/// Where the drop-down of `items` for title `open` is drawn: right under its
/// title, kept inside `bounds`.
pub(super) fn panel_at(spans: &[TitleSpan], open: usize, items: &[Item], bounds: Rect) -> Rect {
    let (x0, x1) = spans.get(open).map_or((bar_x0(), bar_x0()), |s| (s.x0, s.x1));
    place(items, Rect { x: x0, y: 0, w: x1 - x0, h: titlebar_h() }, bounds)
}

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

//! Where each control sits, in the order a click has always tried them.

use alloc::vec::Vec;

use nonos_toolkit::widgets::Rect;

use super::frame::geom::nav_item;
use super::frame::region::{body, grid_half, list_half, search, view_toggle};
use super::layout::layout;
use super::player::transport::transport;
use super::screen::{Route, NAV};
use super::target::Target::{self, *};
use super::view::list_geom::{grid_capacity, list_at, row_rect, rows_visible, tile_rect};
use super::view::page_geom::content;
use super::view::rail_geom::{open_button, rail_row, RAIL_ROWS};

/// What the window shows that decides where its controls sit.
#[derive(Clone, Copy, Debug)]
pub struct Shown {
    pub route: Route,
    pub dims: (u32, u32),
    pub grid: bool,
    /// The first video on the page, and how many the list holds.
    pub scroll: usize,
    pub len: usize,
}

/// Every control on show and the box it is drawn in, but the Details tabs.
pub fn targets(s: &Shown) -> Vec<(Target, Rect)> {
    let (w, h) = s.dims;
    if s.route == Route::Player {
        let l = layout(w, h);
        let (t, head) = (transport(&l, w), [(Back, l.back), (Info, l.info)]);
        let bar = [(Play, t.play), (Prev, t.prev), (Next, t.next), (Seek, l.scrub)];
        return head.into_iter().chain(bar).chain([(Frame, l.video)]).collect();
    }
    let mut out: Vec<(Target, Rect)> = (0..NAV.len()).map(|i| (Nav(i), nav_item(w, i))).collect();
    let page = body(w, h);
    if s.route == Route::Files {
        out.extend((0..RAIL_ROWS).map(|i| (Folder(i), rail_row(page, i))));
        out.extend((s.len > 0).then(|| (Open, open_button(page))));
    }
    let Some(area) = content(s.route, page) else { return out };
    let v = view_toggle(w, h);
    out.extend([(View(true), grid_half(v)), (View(false), list_half(v)), (Search, search(w, h))]);
    out.extend(videos(s, area));
    out
}

/// The tiles or rows on the page, each named by its video's index.
fn videos(s: &Shown, area: Rect) -> Vec<(Target, Rect)> {
    let (first, left) = (s.scroll.min(s.len), s.len.saturating_sub(s.scroll));
    if s.grid {
        let n = grid_capacity(area).min(left);
        return (0..n).map(|k| (Tile(first + k), tile_rect(area, k))).collect();
    }
    let at = list_at(area);
    (0..rows_visible(at).min(left)).map(|k| (Row(first + k), row_rect(at, k))).collect()
}

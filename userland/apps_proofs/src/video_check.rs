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

//! What the video player's proofs share: the windows it can show, the points
//! of a box they test, and the hover fed as the window feeds it.

use nonos_app_skeleton::InputEvent;
use nonos_app_skeleton::InputKind::{self, ButtonUp};
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::widgets::{Cell, Col, Rect, Table};

use crate::ui::frame::region::body;
use crate::ui::hover::Hover;
use crate::ui::screen::Route;
use crate::ui::targets::Shown;
use crate::ui::view::list_geom::list_at;
use crate::ui::view::page_geom::content;

/// `w` by `h` logical pixels at the display scale in force.
pub fn dims(w: u32, h: u32) -> (u32, u32) {
    (scale().px(w), scale().px(h))
}

/// A 960 by 600 window, the size it opens at, on `route` with 40 videos, at
/// the display scale in force.
pub fn window(route: Route) -> Shown {
    Shown { route, dims: dims(960, 600), grid: true, scroll: 0, len: 40 }
}

/// Every route at the size it opens at, full screen and with the sidebar
/// folded, at the display scale in force; the list as well as the grid,
/// scrolled, and with no video.
pub fn windows() -> Vec<Shown> {
    let lists = [(true, 0, 40), (false, 0, 40), (false, 7, 40), (true, 0, 0)];
    let mut all = Vec::new();
    for route in [Route::Library, Route::Files, Route::Details, Route::Player] {
        for dims in [dims(960, 600), dims(1440, 900), dims(800, 560)] {
            all.extend(lists.map(|(grid, scroll, len)| Shown { route, dims, grid, scroll, len }));
        }
    }
    all
}

/// The middle and the four corners of `r`.
pub fn points(r: Rect) -> [(i32, i32); 5] {
    let (x, y, x1, y1) = (r.x as i32, r.y as i32, (r.x + r.w) as i32 - 1, (r.y + r.h) as i32 - 1);
    [(x + r.w as i32 / 2, y + r.h as i32 / 2), (x, y), (x1, y), (x, y1), (x1, y1)]
}

/// Whether two boxes share a pixel.
pub fn meets(a: Rect, b: Rect) -> bool {
    a.x < b.x + b.w && b.x < a.x + a.w && a.y < b.y + b.h && b.y < a.y + a.h
}

/// Feeds the event and names the control under the pointer again, as the
/// window does; true when the window would repaint.
pub fn feed(h: &mut Hover, s: &Shown, kind: InputKind, (x, y): (i32, i32)) -> bool {
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    let ev = InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns };
    let fed = h.feed(&ev);
    let held = kind == ButtonUp && h.over.is_some();
    fed && (h.settle(s) || held)
}

/// Where the page draws its Table.
pub fn table(s: &Shown) -> Rect {
    list_at(content(s.route, body(s.dims.0, s.dims.1)).unwrap())
}

/// The row the Table at `at`, given `n` rows from `first`, has at a point.
pub fn hit(at: Rect, (first, n): (usize, usize), (x, y): (i32, i32)) -> Option<usize> {
    let (cols, blank) = ([Col::new("Name")], vec![&[][..]; n]);
    match Table::new(&cols).rows(&blank).first(first).hit(at, x, y)? {
        Cell::Row { row, .. } => Some(row),
        Cell::Header(_) => None,
    }
}

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

//! The video player's window grown by the display scale still holds every
//! part it lays out: the sidebar folds at the scaled width, the pages, tiles,
//! list rows, Folders rail and Open button sit inside the page, and the
//! player's header, picture and transport inside the window. Its own binary,
//! so setting the scale races no other test.

use apps_proofs::ui::frame::geom::{collapsed, nav_item, sidebar_w, storage};
use apps_proofs::ui::frame::region::{body, content, search, view_toggle};
use apps_proofs::ui::layout::layout;
use apps_proofs::ui::player::transport::transport;
use apps_proofs::ui::screen::Route;
use apps_proofs::ui::view::list_geom::{columns, grid_capacity, list_at, rows_visible};
use apps_proofs::ui::view::page_geom::content as videos;
use apps_proofs::ui::view::rail_geom::{folder_area, open_button, rail_row, RAIL_ROWS};
use apps_proofs::video_check::dims;
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::Rect;

fn inside(a: Rect, b: Rect) -> bool {
    a.x >= b.x && a.y >= b.y && a.x + a.w <= b.x + b.w && a.y + a.h <= b.y + b.h
}

#[test]
fn the_window_grown_by_the_scale_holds_every_part() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        assert!(collapsed(dims(800, 560).0) && !collapsed(dims(960, 600).0), "q={q}: the fold");
        for (w, h) in [dims(960, 600), dims(1440, 900), dims(800, 560)] {
            let (win, page) = (Rect { x: 0, y: 0, w, h }, body(w, h));
            assert!(inside(page, content(w, h)) && inside(content(w, h), win), "q={q}");
            assert_eq!(content(w, h).x, sidebar_w(w), "q={q}: the page starts past the sidebar");
            let v = view_toggle(w, h);
            assert!(search(w, h).x + search(w, h).w <= v.x && inside(v, win), "q={q}: tools");
            if !collapsed(w) {
                assert!(nav_item(w, 1).y + nav_item(w, 1).h <= storage(w, h).y, "q={q}: storage");
            }
            let lib = videos(Route::Library, page).unwrap();
            assert!(columns(lib.w) >= 2 && grid_capacity(lib) >= 4, "q={q}: the grid");
            assert!(rows_visible(list_at(lib)) >= 5, "q={q}: the list");
            let last = rail_row(page, RAIL_ROWS - 1);
            assert!(inside(last, page) && inside(folder_area(page), page), "q={q}: the rail");
            assert!(inside(open_button(page), page), "q={q}: Open");
            let l = layout(w, h);
            let t = transport(&l, w);
            assert!(l.video.h > 0 && l.back.y + l.back.h <= l.video.y, "q={q}: the picture");
            assert!(inside(l.bar, win) && inside(l.scrub, l.bar), "q={q}: the bar");
            for r in [t.prev, t.play, t.next] {
                assert!(inside(r, l.bar), "q={q}: {r:?} leaves the bar");
            }
        }
    }
    set_scale(4);
}

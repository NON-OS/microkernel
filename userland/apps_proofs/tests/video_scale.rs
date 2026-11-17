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

//! Every size the video player lays out with is its 1x size at the display
//! scale, read once: the sidebar, top bar, tiles, list, Folders rail, Details
//! strip, the player's header and transport, and the type ramp. Its own
//! binary, so setting the scale races no other test.

use apps_proofs::ui::frame::geom::{brand, nav_item, sidebar_w, storage};
use apps_proofs::ui::frame::region::{body, grid_half, search, topbar, view_toggle};
use apps_proofs::ui::layout::{edge, header_h, layout, pad, seek_reach, top};
use apps_proofs::ui::player::transport::transport;
use apps_proofs::ui::text::{body_px, head_px, line_h, title_px};
use apps_proofs::ui::view::list_geom::{list_at, tile_rect, track};
use apps_proofs::ui::view::page_geom::{heading, strip};
use apps_proofs::ui::view::rail_geom::{open_button, rail_row};
use apps_proofs::video_check::dims;
use nonos_toolkit::design::scale::{scale, set_scale};

#[rustfmt::skip]
fn sizes() -> (Vec<u32>, Vec<f32>) {
    let ((w, h), (narrow, _)) = (dims(960, 600), dims(800, 560));
    let (page, l, v) = (body(w, h), layout(w, h), view_toggle(w, h));
    let (t, tile, row) = (transport(&l, w), tile_rect(page, 0), rail_row(page, 0));
    let lengths = vec![
        top(), edge(), pad(), header_h(), line_h(), sidebar_w(w), sidebar_w(narrow),
        nav_item(w, 0).h, nav_item(w, 0).x, brand(w).h, storage(w, h).h, topbar(w, h).h,
        search(w, h).h, v.w, grid_half(v).w, tile.w, tile.h, page.w - list_at(page).w,
        track(page).w, row.w, row.h, row.y - page.y, open_button(page).w, open_button(page).h,
        strip(page).h, heading(page).h, l.bar.h, l.scrub.h, l.back.w, l.info.h, t.prev.w,
        t.play.h, l.scrub.y - seek_reach(l.scrub).y,
    ];
    (lengths, vec![body_px(), title_px(), head_px()])
}

#[test]
fn every_size_is_its_1x_size_at_every_scale() {
    set_scale(4);
    let (lengths, fonts) = sizes();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let s = scale();
        let want = (
            lengths.iter().map(|&n| s.px(n)).collect(),
            fonts.iter().map(|&f| s.font(f)).collect(),
        );
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
    }
    set_scale(4);
}

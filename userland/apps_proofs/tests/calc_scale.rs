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

//! The calculator keeps logical metrics and draws them at the display scale:
//! at 1x, 1.25x, 1.5x and 2x every shared size is its 1x size through
//! `scale().px` or `scale().font`, and every fixed-size control is its 1x
//! size the same way. Its own binary, so setting the scale races no other test.

use apps_proofs::calc::manifest::WIDTH;
use apps_proofs::calc::ui::metrics::{metrics, Metrics};
use apps_proofs::calc::ui::{bits_geom, convert_geom, history_geom, nav_geom};
use nonos_toolkit::design::scale::{scale, set_scale};

fn px(v: i32) -> i32 {
    scale().px(v as u32) as i32
}

fn lengths(m: &Metrics) -> [i32; 11] {
    let rail = [m.rail_w, m.rail_pad_x, m.brand_top, m.brand_h, m.nav_top, m.nav_h, m.nav_gap];
    let [a, b, c, d, e, f, g] = rail;
    [a, b, c, d, e, f, g, m.pane_pad, m.readout_h, m.key_gap, m.r_key]
}

fn every_size_is_its_1x_size(one: &Metrics) {
    let (m, s) = (metrics(), scale());
    assert_eq!(lengths(&m), lengths(one).map(px));
    assert_eq!((m.rail_x, m.rail_card_w), (m.pane_pad, m.rail_w - m.pane_pad));
    let fonts = [one.px_title, one.px_body, one.px_mono].map(|f| s.font(f));
    assert_eq!([m.px_title, m.px_body, m.px_mono], fonts);
    assert_eq!(nav_geom::row(0).h as i32, px(one.nav_h));
    assert_eq!(history_geom::row(px(WIDTH as i32), 0).h as i32, px(44));
    assert_eq!(bits_geom::cell(px(WIDTH as i32), 0).h as i32, px(26));
    let swap = convert_geom::swap(px(WIDTH as i32));
    assert_eq!((swap.w as i32, swap.h as i32), (px(92), px(36)));
    assert_eq!(convert_geom::chip(px(WIDTH as i32), 0).h as i32, px(36));
    assert_eq!(convert_geom::result(px(WIDTH as i32), 0).h as i32, px(72));
}

#[test]
fn every_size_is_its_1x_size_at_every_scale() {
    set_scale(4);
    let one = metrics();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        every_size_is_its_1x_size(&one);
    }
    set_scale(4);
}

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

//! Every size the window frame lays out with, toolbar, address text, tab strip
//! and sidebar, is its 1x size at the display scale, and a window grown by the
//! scale fits. Its own binary, so setting the scale races no other test.

use capsule_browser_proofs::browser::omnibox::geometry::{pill_text, text_px, TEXT_PX};
use capsule_browser_proofs::browser::ui::layout::{compute, Layout, SidebarMode};
use capsule_browser_proofs::browser::ui::sidebar_geom::{brand, rows};
use capsule_browser_proofs::browser::ui::tabs_geom::{plus_rect, tab_rect, Strip};
use capsule_browser_proofs::browser::ui::toolbar_geom::buttons;
use nonos_toolkit::design::scale::{scale, set_scale};

const W: u32 = 1360;
const H: u32 = 760;

fn window(w: u32, mode: SidebarMode) -> Layout {
    compute(scale().px(w) as i32, scale().px(H) as i32, mode)
}

#[rustfmt::skip]
fn sizes() -> Vec<i32> {
    let (wide, narrow) = (window(W, SidebarMode::Auto), window(1179, SidebarMode::Auto));
    let (bar, side) = (wide.toolbar, window(1400, SidebarMode::Full).sidebar);
    let g = buttons(bar);
    let s = Strip { n: 2, active: 0, w: scale().px(900) as i32, h: scale().px(40) as i32 };
    let (t0, t1, r) = (tab_rect(&s, 0).unwrap(), tab_rect(&s, 1).unwrap(), rows(false, side, 2));
    let notice = (pill_text(bar, false).w - pill_text(bar, true).w) as i32;
    vec![
        bar.h, wide.sidebar.w, narrow.sidebar.w, g.back.x, g.back.y, g.back.w, g.back.h,
        g.fwd.x - g.back.right(), g.pill.x - g.home.right(), g.text.x - g.pill.x,
        bar.right() - g.sidebar.right(), notice, t0.w, t0.h, t1.x - t0.right(),
        plus_rect(&s).w, r[0].rect.x - side.x, r[0].rect.h, r[1].rect.y - r[0].rect.bottom(),
        brand(false, side).y - side.y,
    ]
}

fn fits(q: u32) {
    let (w, h, l) = (scale().px(W) as i32, scale().px(H) as i32, window(W, SidebarMode::Auto));
    assert!(!l.rail && window(1179, SidebarMode::Auto).rail, "q={q}: the fold moved");
    let ends = (l.toolbar.right(), l.content.right(), l.content.bottom(), l.sidebar.right());
    assert_eq!(ends, (w, w, h, l.content.x), "q={q}: the window does not fill its frame");
    let g = buttons(l.toolbar);
    let all = [g.back, g.fwd, g.reload, g.home, g.pill, g.reader, g.star, g.tabs, g.sidebar];
    let row = all.windows(2).all(|p| p[0].right() <= p[1].x);
    assert!(row && all[0].x >= 0 && all[8].right() <= w && all[0].bottom() <= l.toolbar.h, "q={q}");
}

#[test]
fn every_frame_size_is_its_1x_size_and_the_window_fits_at_every_scale() {
    set_scale(4);
    let lengths = sizes();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let want: Vec<i32> = lengths.iter().map(|&n| scale().px(n as u32) as i32).collect();
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
        assert_eq!(text_px(), scale().font(TEXT_PX), "q={q}");
        fits(q);
    }
    set_scale(4);
}

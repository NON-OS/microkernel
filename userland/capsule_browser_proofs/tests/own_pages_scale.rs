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

//! Settings' band and the reader's columns are their 1x sizes at the display scale, both
//! pages fit a window grown by it, and no control takes a press under the band.

use capsule_browser_proofs::browser::settings::{controls::paint, look::Shows};
use capsule_browser_proofs::browser::ui::layout::{compute, SidebarMode};
use capsule_browser_proofs::browser::ui::reader_column::{column, wide_min};
use capsule_browser_proofs::browser::ui::reader_geom::panel;
use capsule_browser_proofs::browser::ui::settings_geom::{band_h, Kind, SettingsGeom};
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::{paint::PaintBuffer, widgets::Controls};

fn content() -> (i32, i32) {
    let c = compute(scale().px(1360) as i32, scale().px(760) as i32, SidebarMode::Full).content;
    (c.w, c.h)
}

fn sizes() -> Vec<i32> {
    let ((w, h), wide) = (content(), scale().px(1600) as i32);
    let first = SettingsGeom::new(w, h, 0).items[0].rect;
    vec![band_h(), first.y, column(wide, 0).1, column(wide, 1).1, column(wide, 2).1]
}

fn presses_only_below_the_band(q: u32, (w, h): (i32, i32), scroll: i32) {
    let (g, (wu, hu), mut c) =
        (SettingsGeom::new(w, h, scroll), (w as u32, h as u32), Controls::new());
    assert!(g.items.iter().all(|it| it.rect.x >= 0 && it.rect.right() <= w), "q={q}: too wide");
    let mut px = vec![0u32; (wu * hu) as usize];
    let fb = &mut PaintBuffer { pixels: &mut px, stride_words: wu, width: wu, height: hu };
    let had = Shows { size: 1, network: Some(0), proxy: true, history: true, ..Shows::default() };
    paint(fb, &g, h, &Shows { downloads: true, bookmarks: 2, ..had }, &mut c);
    let placed = g.items.iter().filter(|it| matches!(it.kind, Kind::Ctl(_)));
    for r in placed.map(|it| it.rect).filter(|r| r.bottom() > 0 && r.y < h) {
        let under = (r.y.max(0)..r.bottom().min(band_h())).all(|y| c.at(r.x + 1, y).is_none());
        let over = (band_h().max(r.y)..r.bottom().min(h)).all(|y| c.at(r.x + 1, y).is_some());
        assert!(under && over, "q={q}: {r:?} takes presses off what it draws");
    }
}

fn fits(q: u32) {
    let ((w, h), step) = (content(), scale().px(9) as usize);
    (0..scale().px(900) as i32)
        .step_by(step)
        .for_each(|s| presses_only_below_the_band(q, (w, h), s));
    let (x, cw) = column(w, 1);
    assert!(w >= wide_min() && x + cw <= panel(w).panel.x && panel(w).panel.right() <= w, "q={q}");
}

#[test]
fn every_own_page_size_is_its_1x_size_and_the_pages_fit_at_every_scale() {
    set_scale(4);
    let lengths = sizes();
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let want: Vec<i32> = lengths.iter().map(|&n| scale().px(n as u32) as i32).collect();
        assert_eq!(sizes(), want, "q={q}: a size is not its 1x size at the scale");
        fits(q);
    }
    set_scale(4);
}

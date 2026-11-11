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

//! The start page's sizes are its 1x sizes at the display scale, it fits a window grown
//! by the scale, and a favourite takes presses where it is drawn. Its own binary.

use capsule_browser_proofs::browser::start::{fav_list::Fav, favorites};
use capsule_browser_proofs::browser::ui::layout::{compute, SidebarMode};
use capsule_browser_proofs::browser::ui::start_geom::{fav_at, fav_rect, pill_text};
use capsule_browser_proofs::browser::ui::start_geom::{scrolled, StartGeom};
use nonos_toolkit::design::scale::{scale, set_scale};
use nonos_toolkit::{paint::PaintBuffer, theme::Category};

fn page(favs: usize, scroll: i32) -> (i32, i32, StartGeom) {
    let c = compute(scale().px(1360) as i32, scale().px(760) as i32, SidebarMode::Full).content;
    (c.w, c.h, scrolled(c.w, c.h, favs, scroll))
}

#[rustfmt::skip]
fn sizes() -> Vec<i32> {
    let (_, _, g) = page(5, 0);
    let (p, t, f, [a, b, _]) = (g.pill, pill_text(&g).unwrap(), fav_rect(&g, 0), g.cards);
    vec![
        p.w, p.h, g.hero.bottom() - p.bottom(), p.y - g.greet_y, p.y - g.sub_y,
        g.fav_title_y - g.hero.bottom(), g.tiles_y - g.fav_title_y, f.w, f.h,
        t.x - p.x, p.right() - t.right(), a.h, b.x - a.right(), a.y - f.bottom(),
    ]
}

fn drawn(g: &StartGeom, w: u32, h: u32) -> Vec<u32> {
    let site = |i| Fav { label: "S".into(), url: "".into(), letter: 'S', color: Category::new(i) };
    let (favs, mut px) = ((0..8).map(site).collect::<Vec<_>>(), vec![0u32; (w * h) as usize]);
    let fb = &mut PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
    favorites::paint(g, &favs, fb);
    px
}

fn fits(q: u32) {
    let ((w, h, top), (_, _, end), bottom) = (page(8, 0), page(8, i32::MAX / 2), scale().px(32));
    let spots = (0..8).map(|i| fav_rect(&top, i)).chain(top.cards).chain([top.pill]);
    assert!(spots.clone().all(|r| r.x >= 0 && r.right() <= w), "q={q}: the page is too wide");
    let (last, full) = (end.cards[2].bottom(), top.cards[2].bottom() + bottom as i32);
    assert_eq!(last + bottom as i32, full.min(h), "q={q}: the end is off");
    let ink = drawn(&top, w as u32, h as u32).into_iter().enumerate().filter(|&(_, c)| c != 0);
    for (x, y) in ink.map(|(n, _)| (n as i32 % w, n as i32 / w)).filter(|p| p.1 >= top.tiles_y) {
        let hit = fav_at(&top, x, y).map(|i| fav_rect(&top, i));
        assert!(hit.is_some_and(|r| r.contains(x, y)), "q={q}: ink at {x},{y} takes no press");
    }
}

#[test]
fn every_start_size_is_its_1x_size_and_the_page_fits_at_every_scale() {
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

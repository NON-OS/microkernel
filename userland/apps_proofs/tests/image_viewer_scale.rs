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

//! The image viewer keeps logical metrics and draws them at the display scale:
//! at 1x, 1.25x, 1.5x and 2x the gallery's tiles and header and the prev and
//! next buttons are their 1x sizes through `scale().px`, a swipe needs the
//! scaled distance, and in a window grown by the same scale the gallery still
//! reaches every image, keeps only the thumbnails near the view and follows the
//! keyboard's selection, and a click still steps where it stepped before. Its
//! own binary, so setting the scale races no other test.

use apps_proofs::viewer::gallery::layout::{
    cell_rect, follow, full_rows, grid, header_h, kept, max_scroll, thumb_h, thumb_w, visible,
};
use apps_proofs::viewer::manifest::manifest;
use apps_proofs::viewer::nav::{button_rects, swipe_delta};
use apps_proofs::viewer::target::{target_at, Shown, Target};
use nonos_toolkit::design::scale::{scale, set_scale};

const SCALES: [u32; 4] = [4, 5, 6, 8];

fn window() -> (u32, u32) {
    let (m, s) = (manifest(), scale());
    (s.px(m.width), s.px(m.height))
}

fn every_size_is_its_1x_size_at_the_scale() {
    set_scale(4);
    let (w1, h1) = window();
    let (l1, _) = button_rects(w1, h1);
    let (g1, cell1) = (grid(w1), cell_rect(0, 0, &grid(w1)));
    let tile1 = (thumb_w(), thumb_h(), header_h());
    for q in SCALES {
        set_scale(q);
        let (s, (w, h)) = (scale(), window());
        let (l, r) = button_rects(w, h);
        assert_eq!((l.x, l.w, l.h), (s.px(l1.x), s.px(l1.w), s.px(l1.h)), "q={q}");
        assert_eq!((l.y, r.y, r.x + r.w + l.x), ((h - l.h) / 2, l.y, w), "q={q}");
        let tile = (s.px(tile1.0), s.px(tile1.1), s.px(tile1.2));
        assert_eq!((thumb_w(), thumb_h(), header_h()), tile, "q={q}");
        let g = grid(w);
        assert_eq!((g.cell_w, g.cell_h), (s.px(g1.cell_w), s.px(g1.cell_h)), "q={q}");
        let px = |n: i32| s.px(n as u32) as i32;
        let cell = cell_rect(0, 0, &g);
        assert_eq!(cell, (px(cell1.0), px(cell1.1), s.px(cell1.2), s.px(cell1.3)), "q={q}");
        assert!(g.cols >= g1.cols && cell.0 as u32 + g.cols * g.cell_w <= w, "q={q}");
        let far = s.px(80) as i32;
        let swipes = [far, far + 1, -far - 1].map(|dx| swipe_delta(dx, 0, false));
        assert_eq!(swipes, [None, Some(-1), Some(1)], "q={q}: a swipe needs the scaled distance");
    }
}

fn the_gallery_reaches_every_image_the_store_holds(w: u32, h: u32) {
    let g = grid(w);
    for count in [1usize, 60, 61, 143, 5_000] {
        let last_page = max_scroll(count, h, &g);
        let (_, end) = visible(count, last_page, h, &g);
        assert_eq!(end, count, "{count} images: the last page ends at {end}");
    }
}

fn the_thumbnails_kept_do_not_grow_with_the_store(w: u32, h: u32) {
    let g = grid(w);
    let cols = g.cols as usize;
    let rows_on_screen = visible(usize::MAX / 2, 0, h, &g).1 / cols;
    for scroll in [0usize, 7, 300] {
        let (from, to) = kept(10_000, scroll, h, &g);
        assert!(to - from <= (rows_on_screen + 2) * cols, "{from}..{to}");
        let (show_from, show_to) = visible(10_000, scroll, h, &g);
        assert!(from <= show_from && show_to <= to);
    }
}

fn the_view_follows_the_selection_down_and_back_up(w: u32, h: u32) {
    let g = grid(w);
    let cols = g.cols as usize;
    let count = 200;
    let shown = full_rows(h, &g);
    let sel = 10 * cols;
    let scroll = follow(0, sel, count, h, &g);
    assert_eq!(scroll, 10 + 1 - shown, "ten rows down is the last whole row");
    let (from, to) = visible(count, scroll, h, &g);
    assert!(from <= sel && sel < to);
    assert_eq!(follow(scroll, 0, count, h, &g), 0, "the first image goes back to the top");
    assert_eq!(follow(scroll, sel, count, h, &g), scroll, "one on screen does not move the view");
}

fn the_gallery_scrolls_alike_at_every_scale() {
    for q in SCALES {
        set_scale(q);
        let (w, h) = window();
        the_gallery_reaches_every_image_the_store_holds(w, h);
        the_thumbnails_kept_do_not_grow_with_the_store(w, h);
        the_view_follows_the_selection_down_and_back_up(w, h);
    }
}

/// The hit test nav.rs had before the shared `Rect`, written out again with
/// its numbers grown by the scale.
fn old_hit(x: i32, y: i32, vw: u32, vh: u32) -> Option<i32> {
    let px = |n: u32| scale().px(n);
    let top = (vh.saturating_sub(px(64)) / 2) as i32;
    let inside = |rx: i32| x >= rx && x < rx + px(40) as i32 && y >= top && y < top + px(64) as i32;
    let sides = [(px(8) as i32, -1), (vw.saturating_sub(px(8) + px(40)) as i32, 1)];
    sides.into_iter().find(|&(rx, _)| inside(rx)).map(|(_, d)| d)
}

fn a_click_steps_where_it_stepped_before_at_every_scale() {
    for q in SCALES {
        set_scale(q);
        let (w, h) = window();
        let s = Shown { gallery: false, images: 2, w, h, scroll: 0, tiles: 0 };
        for y in (-2..h as i32 + 2).step_by(3) {
            for x in -2..w as i32 + 2 {
                let now = target_at(&s, x, y).and_then(Target::step);
                assert_eq!(now, old_hit(x, y, w, h), "q={q}: ({x}, {y}) in {w}x{h}");
            }
        }
    }
}

/// One test, so no other thread sets the scale under it.
#[test]
fn the_viewer_scales_and_still_steps_and_scrolls_alike() {
    every_size_is_its_1x_size_at_the_scale();
    the_gallery_scrolls_alike_at_every_scale();
    a_click_steps_where_it_stepped_before_at_every_scale();
    set_scale(4);
}

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

//! Proofs for the editor's layout metrics: text sizes stay inside the bars
//! and rows that hold them, the zoom is bounded, and wrapped text stays in
//! its pane, at 1x; layout_scale_tests holds them at every scale.

use nonos_toolkit::design::scale::set_scale;

use crate::editor::square_tests::lock;
use crate::layout::{
    activity_w, body_px, chrome_px, gutter_px, gutter_w, line_height, pad_top, ribbon_h, row_h,
    sidebar_w, tabbar_h, text_left, titlebar_h, wrap_cols, GLYPH_ADVANCE, MAX_SCALE, MIN_SCALE,
};

#[test]
fn chrome_text_fits_the_bar_or_row_it_is_drawn_in() {
    let _g = lock();
    set_scale(4);
    assert!(9.0 + chrome_px() <= tabbar_h() as f32);
    assert!(5.0 + chrome_px() <= row_h() as f32);
    assert!(chrome_px() < titlebar_h() as f32);
    assert!(chrome_px() < ribbon_h() as f32);
}

#[test]
fn the_sidebar_leaves_the_window_most_of_its_width() {
    let _g = lock();
    set_scale(4);
    assert!(activity_w() + sidebar_w() < 1024 / 3);
    assert!(wrap_cols(1024 - activity_w() - sidebar_w(), GLYPH_ADVANCE) >= 60);
}

#[test]
fn the_zoom_is_held_between_its_bounds() {
    let _g = lock();
    set_scale(4);
    assert_eq!(body_px(0), body_px(MIN_SCALE));
    assert_eq!(body_px(u32::MAX), body_px(MAX_SCALE));
    for s in MIN_SCALE..MAX_SCALE {
        assert!(body_px(s) < body_px(s + 1), "each zoom step makes the text larger");
    }
}

#[test]
fn a_line_is_taller_than_its_text_and_the_gutter_is_no_larger() {
    let _g = lock();
    set_scale(4);
    for s in MIN_SCALE..=MAX_SCALE {
        assert!(line_height(s) as f32 > body_px(s), "scale {s}: lines would overlap");
        assert!(pad_top() < line_height(s));
        assert!(gutter_px(s) <= body_px(s));
        assert!(gutter_px(s) >= 9.0, "line numbers stay legible");
    }
}

#[test]
fn text_starts_past_the_gutter_and_wraps_inside_the_pane() {
    let _g = lock();
    set_scale(4);
    for pane_x in [0, 46, 266] {
        assert_eq!(text_left(pane_x), pane_x + gutter_w() + 12);
    }
    for pane_w in [400, 640, 1024, 1920] {
        let cols = wrap_cols(pane_w, GLYPH_ADVANCE);
        assert!(text_left(0) + cols * GLYPH_ADVANCE + 12 <= pane_w, "{pane_w} px");
    }
}

#[test]
fn the_wrap_width_is_bounded_whatever_the_pane() {
    let _g = lock();
    set_scale(4);
    assert_eq!(wrap_cols(0, GLYPH_ADVANCE), 16, "a pane narrower than the gutter");
    assert_eq!(wrap_cols(1 << 20, GLYPH_ADVANCE), 400);
    assert_eq!(wrap_cols(1024, 0), 400, "a zero advance does not divide by zero");
}

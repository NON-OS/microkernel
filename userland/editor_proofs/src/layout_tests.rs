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
//! its pane.

use crate::layout::{
    body_px, gutter_px, line_height, text_left, wrap_cols, ACTIVITY_W, CHROME_PX, FOOTER_H,
    GLYPH_ADVANCE, GUTTER_W, MAX_SCALE, MIN_SCALE, PAD_TOP, PAD_X, RIBBON_H, ROW_H, SIDEBAR_W,
    STATUS_PX, TABBAR_H, TEXT_PAD, TITLEBAR_H,
};

#[test]
fn chrome_text_fits_the_bar_or_row_it_is_drawn_in() {
    // Tab labels are drawn 9 px into the tab bar, sidebar names 5 px into
    // their row, the status line inside the footer.
    const {
        assert!(9.0 + CHROME_PX <= TABBAR_H as f32);
        assert!(5.0 + CHROME_PX <= ROW_H as f32);
        assert!(CHROME_PX < TITLEBAR_H as f32);
        assert!(CHROME_PX < RIBBON_H as f32);
        assert!(STATUS_PX < FOOTER_H as f32);
    };
}

#[test]
fn the_sidebar_leaves_the_window_most_of_its_width() {
    // A 1024 px window with the sidebar open keeps at least 60 columns of
    // text at the default advance.
    const { assert!(ACTIVITY_W + SIDEBAR_W < 1024 / 3) };
    assert!(wrap_cols(1024 - ACTIVITY_W - SIDEBAR_W, GLYPH_ADVANCE) >= 60);
}

#[test]
fn the_zoom_is_held_between_its_bounds() {
    assert_eq!(body_px(0), body_px(MIN_SCALE));
    assert_eq!(body_px(u32::MAX), body_px(MAX_SCALE));
    for s in MIN_SCALE..MAX_SCALE {
        assert!(body_px(s) < body_px(s + 1), "each zoom step makes the text larger");
    }
}

#[test]
fn a_line_is_taller_than_its_text_and_the_gutter_is_no_larger() {
    for s in MIN_SCALE..=MAX_SCALE {
        assert!(line_height(s) as f32 > body_px(s), "scale {s}: lines would overlap");
        assert!(PAD_TOP < line_height(s));
        assert!(gutter_px(s) <= body_px(s));
        assert!(gutter_px(s) >= 9.0, "line numbers stay legible");
    }
}

#[test]
fn text_starts_past_the_gutter_and_wraps_inside_the_pane() {
    for pane_x in [0, 46, 266] {
        assert_eq!(text_left(pane_x), pane_x + GUTTER_W + TEXT_PAD);
    }
    for pane_w in [400, 640, 1024, 1920] {
        let cols = wrap_cols(pane_w, GLYPH_ADVANCE);
        assert!(text_left(0) + cols * GLYPH_ADVANCE + PAD_X <= pane_w, "{pane_w} px");
    }
}

#[test]
fn the_wrap_width_is_bounded_whatever_the_pane() {
    assert_eq!(wrap_cols(0, GLYPH_ADVANCE), 16, "a pane narrower than the gutter");
    assert_eq!(wrap_cols(1 << 20, GLYPH_ADVANCE), 400);
    assert_eq!(wrap_cols(1024, 0), 400, "a zero advance does not divide by zero");
}

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

//! The image viewer's gallery lists every image, keeps only the thumbnails
//! near the view, and follows the keyboard's selection; its overlays say the
//! scale the image is really drawn at and what each key does.

use crate::viewer::caption::{interval_label, nav_shown, size_line, zoom_percent, KEYMAP};
use crate::viewer::gallery::layout::{follow, full_rows, grid, kept, max_scroll, visible};
use crate::viewer::viewport::FitMode;

const W: u32 = 720;
const H: u32 = 520;

/// The gallery used to draw the first sixty images and no more; the rest of
/// the store's images could not be reached at all.
#[test]
fn the_gallery_reaches_every_image_the_store_holds() {
    let g = grid(W);
    for count in [1usize, 60, 61, 143, 5_000] {
        let last_page = max_scroll(count, H, &g);
        let (_, end) = visible(count, last_page, H, &g);
        assert_eq!(end, count, "{count} images: the last page ends at {end}");
    }
}

/// However many images there are, the thumbnails held are those on screen and
/// a row either side.
#[test]
fn the_thumbnails_kept_do_not_grow_with_the_store() {
    let g = grid(W);
    let cols = g.cols as usize;
    let rows_on_screen = visible(usize::MAX / 2, 0, H, &g).1 / cols;
    for scroll in [0usize, 7, 300] {
        let (from, to) = kept(10_000, scroll, H, &g);
        assert!(to - from <= (rows_on_screen + 2) * cols, "{from}..{to}");
        let (show_from, show_to) = visible(10_000, scroll, H, &g);
        assert!(from <= show_from && show_to <= to);
    }
}

#[test]
fn the_view_follows_the_selection_down_and_back_up() {
    let g = grid(W);
    let cols = g.cols as usize;
    let count = 200;
    let shown = full_rows(H, &g);
    // Ten rows down: the view scrolls so that row is the last whole one.
    let sel = 10 * cols;
    let scroll = follow(0, sel, count, H, &g);
    assert_eq!(scroll, 10 + 1 - shown);
    let (from, to) = visible(count, scroll, H, &g);
    assert!(from <= sel && sel < to);
    // Back to the first image: the view goes back to the top.
    assert_eq!(follow(scroll, 0, count, H, &g), 0);
    // A selection already on screen does not move the view.
    assert_eq!(follow(scroll, sel, count, H, &g), scroll);
}

/// The info overlay showed the zoom factor, which is relative to the fit: a
/// 4000 by 3000 photo fitted to the window read "100%" at a sixth of its size.
#[test]
fn the_zoom_shown_is_the_scale_the_image_is_drawn_at() {
    assert_eq!(zoom_percent(FitMode::Fit, 4000, 3000, W, H, 1.0), 17);
    assert_eq!(zoom_percent(FitMode::Fit, 4000, 3000, W, H, 2.0), 35);
    assert_eq!(zoom_percent(FitMode::Actual, 4000, 3000, W, H, 1.0), 100);
    assert_eq!(zoom_percent(FitMode::Actual, 400, 300, W, H, 1.25), 125);
}

/// `[` and `]` step the interval by half a second; it showed as whole seconds,
/// so 3.5 seconds read "3s".
#[test]
fn the_slideshow_interval_reads_as_it_is() {
    assert_eq!(interval_label(3000), "3s");
    assert_eq!(interval_label(3500), "3.5s");
    assert_eq!(interval_label(1000), "1s");
    assert_eq!(interval_label(15000), "15s");
}

/// `[` shortens the interval (a faster slideshow) and `]` lengthens it; the
/// help said the opposite, and did not say how to get back to the gallery.
#[test]
fn the_help_says_what_the_keys_do() {
    let row = |keys: &str| KEYMAP.iter().find(|(k, _)| *k == keys).map(|(_, what)| *what);
    assert_eq!(row("[ / ]"), Some("slideshow faster/slower"));
    assert_eq!(row("Esc/Bksp"), Some("back to gallery"));
    let source = include_str!("../../capsule_image_viewer/src/viewer/app.rs");
    assert!(source.contains("b'[' => st.interval_ms = st.interval_ms.saturating_sub(500)"));
}

#[test]
fn a_file_that_was_not_shown_has_no_size_to_report() {
    assert_eq!(size_line(Some((640, 480)), "PNG", 1234), "640x480  PNG  1234B");
    assert_eq!(size_line(None, "PNG", 0), "not shown  PNG");
}

/// The buttons were hidden when the image on screen failed to decode while
/// their click area stayed live; now one test decides both, and a failed file
/// can be stepped past.
#[test]
fn prev_and_next_show_whenever_there_is_somewhere_to_go() {
    assert!(!nav_shown(0));
    assert!(!nav_shown(1));
    assert!(nav_shown(2));
}

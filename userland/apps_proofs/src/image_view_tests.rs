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

//! The image viewer's overlays say the scale the image is really drawn at and
//! what each key does. Where its gallery scrolls is proven at every display
//! scale in `tests/image_viewer_scale.rs`.

use crate::viewer::caption::{interval_label, nav_shown, size_line, zoom_percent, KEYMAP};
use crate::viewer::manifest::manifest;
use crate::viewer::viewport::FitMode;

/// The info overlay showed the zoom factor, which is relative to the fit: a
/// 4000 by 3000 photo fitted to the window read "100%" at a sixth of its size.
#[test]
fn the_zoom_shown_is_the_scale_the_image_is_drawn_at() {
    let (w, h) = (manifest().width, manifest().height);
    assert_eq!(zoom_percent(FitMode::Fit, 4000, 3000, w, h, 1.0), 17);
    assert_eq!(zoom_percent(FitMode::Fit, 4000, 3000, w, h, 2.0), 35);
    assert_eq!(zoom_percent(FitMode::Actual, 4000, 3000, w, h, 1.0), 100);
    assert_eq!(zoom_percent(FitMode::Actual, 400, 300, w, h, 1.25), 125);
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

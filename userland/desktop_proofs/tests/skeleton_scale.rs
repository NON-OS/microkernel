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

//! The skeleton sets the toolkit's scale when it learns the display, before
//! it opens the window or paints, and for now it sets it to one to one on
//! every display. A test binary of its own, so no other test moves the latch.

use nonos_app_skeleton::runner::chrome::{learn_display, widget_quarters};
use nonos_toolkit::design::scale::{scale, set_scale};

const DISPLAYS: [(u32, u32); 5] =
    [(1280, 720), (1366, 768), (1920, 1080), (2560, 1600), (3840, 2160)];

const BOOT: &str = include_str!("../../app_skeleton/src/runner/boot.rs");
const FIT: &str = include_str!("../../app_skeleton/src/runner/fit_display.rs");

fn at(src: &str, needle: &str) -> usize {
    src.find(needle).unwrap_or_else(|| panic!("{needle} is gone"))
}

#[test]
fn learning_the_display_sets_the_widget_scale_to_one_to_one() {
    for (w, h) in DISPLAYS {
        assert_eq!(widget_quarters(w, h), 4, "{w}x{h}");
        set_scale(8);
        learn_display(w, h);
        assert_eq!(scale().quarters(), 4, "{w}x{h}");
    }
}

#[test]
fn the_scale_is_set_before_the_window_opens_or_paints() {
    let fit = at(BOOT, "fit_to_display(");
    assert!(fit < at(BOOT, "open_window("));
    assert!(fit < at(BOOT, "prime_frame(&mut app"));
    assert!(at(FIT, "learn_display(") < at(FIT, "chrome_growth_at("));
}

#[test]
fn the_content_area_grows_by_the_widget_scale_before_the_frame() {
    let grow = at(FIT, "s.px(manifest.width)");
    assert!(at(FIT, "learn_display(") < at(FIT, "let s = scale();"));
    assert!(at(FIT, "let s = scale();") < grow);
    assert!(at(FIT, "s.px(manifest.height)") < at(FIT, "chrome_growth_at("));
    assert!(grow < at(FIT, "WINDOW_FRACTION / 100"));
}

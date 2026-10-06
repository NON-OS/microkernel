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

//! The one scale: derived from the canvas alone, so a panel shows setup at
//! the size a panel of its canvas's size would, and the type ramp it
//! multiplies.

use crate::displays::{canvases, DISPLAYS};
use crate::scale::{
    Scale, BODY_PX, CAPTION_PX, HEADING_PX, LABEL_PX, LEAD_PX, TITLE_PX, TYPE_RATIO, UNIT,
};

/// Screen pixels per logical pixel on a canvas: the compositor's doubling
/// times setup's own scale, in quarters.
fn on_screen(c: &crate::displays::Canvas) -> u32 {
    c.compositor * Scale::for_canvas(c.width, c.height).quarters()
}

#[test]
fn a_display_shows_setup_as_its_canvas_size_asks() {
    // (display, quarters on screen through the half-size canvas, and through
    // the full-size canvas the compositor falls back to)
    let want = [
        ((1366, 768), 4, 4),
        ((1440, 900), 4, 4),
        ((1920, 1080), 5, 5),
        ((2560, 1440), 8, 6),
        ((2560, 1600), 8, 6),
        ((3840, 2160), 10, 8),
    ];
    let all = canvases();
    for (display, halved, full) in want {
        let mine: Vec<_> = all.iter().filter(|c| c.display == display).collect();
        assert_eq!(on_screen(mine[0]), halved, "{display:?} through the compositor's canvas");
        let fallback = mine.get(1).map_or(halved, |c| on_screen(c));
        assert_eq!(fallback, full, "{display:?} on a full-size canvas");
    }
    for c in &all {
        let q = on_screen(c);
        assert!((4..=10).contains(&q), "{c:?}: {q} quarters is under 1x or over 2.5x");
    }
    assert_eq!(DISPLAYS.len(), 6);
}

#[test]
fn a_half_size_canvas_is_drawn_as_a_panel_of_its_size_is() {
    for c in canvases().iter().filter(|c| c.compositor == 2) {
        let panel = canvases().into_iter().find(|p| p.compositor == 1 && (p.width, p.height) == (c.width, c.height));
        let alone = Scale::for_canvas(c.width, c.height);
        if let Some(p) = panel {
            assert_eq!(alone, Scale::for_canvas(p.width, p.height), "{c:?}");
        }
        assert!(alone.quarters() <= 5, "{c:?}: a halved canvas is never drawn past 1.25");
    }
}

#[test]
fn the_scale_grows_with_the_short_side_only() {
    assert_eq!(Scale::for_canvas(3440, 1439).quarters(), 5, "an ultrawide below 1440 high");
    assert_eq!(Scale::for_canvas(1440, 900), Scale::ONE, "a panel under 1000 high");
    assert_eq!(Scale::for_canvas(3440, 1440).quarters(), 6);
    assert_eq!(Scale::for_canvas(1440, 2560).quarters(), 6, "portrait reads its short side");
    assert_eq!(Scale::for_canvas(5120, 2880).quarters(), 8);
    let mut last = 0;
    for short in (600..3000).step_by(20) {
        let q = Scale::for_canvas(short * 2, short).quarters();
        assert!(q >= last, "never shrinks as the canvas grows");
        last = q;
    }
}

#[test]
fn pixels_and_units_round_to_the_nearest_pixel() {
    let half = Scale::for_canvas(2560, 1440);
    assert_eq!(half.px(1), 2, "1.5 rounds to 2");
    assert_eq!(half.px(2), 3);
    assert_eq!(half.units(1), 12);
    assert_eq!(Scale::ONE.units(5), 5 * UNIT);
    assert_eq!(Scale::for_canvas(3840, 2160).units(3), 48);
    assert_eq!(Scale::ONE.px(u32::MAX), u32::MAX / 4, "saturates instead of wrapping");
    assert_eq!(half.font(BODY_PX), 25.5);
}

#[test]
fn the_type_sizes_are_one_ramp() {
    let step = |k: i32| (BODY_PX * TYPE_RATIO.powi(k)).round();
    assert_eq!(LABEL_PX, step(-2));
    assert_eq!(CAPTION_PX, step(-1));
    assert_eq!(HEADING_PX, step(1));
    assert_eq!(LEAD_PX, step(2));
    assert_eq!(TITLE_PX, step(4));
    const { assert!(TITLE_PX >= 2.0 * BODY_PX, "a title reads at a glance against the body") }
}

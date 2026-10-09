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

//! The desktop's type and geometry scale as first-boot setup's do on the same
//! canvas: 1.25 from a short side of 1000, 1.5 from 1440, 2 from 2160, read
//! off the canvas the compositor hands out (half the panel from 2560 by 1440).

use crate::shell_scale::quarters_for;
use crate::ui_font::{px, scale, scaled, set_scale};

#[test]
fn the_scale_follows_the_canvas_short_side() {
    // (canvas, quarters): 1366x768 and the half canvas of a 2560x1440 panel
    // draw one to one; a 1080p canvas, or a 4K panel's half canvas, at 1.25.
    let cases = [
        ((1366, 768), 4),
        ((1280, 720), 4),
        ((1280, 800), 4),
        ((1920, 1080), 5),
        ((1920, 1200), 5),
        ((2560, 1600), 6),
        ((2880, 1620), 6),
        ((3840, 2160), 8),
    ];
    for ((w, h), q) in cases {
        assert_eq!(quarters_for(w, h), q, "{w}x{h}");
    }
}

/*
 * One test, since the scale is a latch every size reads: tests run on
 * threads at once, and two setting it would race.
 */
#[test]
fn sizes_and_type_grow_together() {
    set_scale(4);
    assert_eq!((px(46), px(7), scale()), (46, 7, 1));
    assert_eq!(scaled(13.0), 13.0);

    set_scale(5);
    // The menu bar, a dock slot and its gap, at 1.25, to the nearest pixel.
    assert_eq!((px(46), px(7), px(1)), (58, 9, 1));
    assert_eq!(scaled(13.0), 16.25);
    // Glyphs drawn in whole pixels stay at one until the scale is two.
    assert_eq!(scale(), 1);

    set_scale(6);
    assert_eq!((px(46), scale()), (69, 1));
    assert_eq!(scaled(12.0), 18.0);

    set_scale(8);
    assert_eq!((px(46), scale()), (92, 2));
    assert_eq!(scaled(13.5), 27.0);

    // Below one to one is never asked for; a zero is read as one.
    set_scale(0);
    assert_eq!((px(46), scale()), (46, 1));

    // The app runtime keeps windows below the bar and clear of the dock the
    // shell draws, so its two numbers are the shell's own at every scale.
    use crate::runner::chrome::{dock_for, menubar_for};
    for (w, h) in [(1280, 800), (1920, 1080), (2560, 1600), (3840, 2160)] {
        set_scale(quarters_for(w, h));
        assert_eq!(menubar_for(w, h), px(46), "{w}x{h}");
        assert_eq!(dock_for(w, h), px(64) + px(16), "{w}x{h}");
        // A new window is placed in the work area: from the foot of the bar
        // to the top of the dock's band, the full width.
        let (x, y, ww, wh) = crate::runner::chrome::work_area(w, h);
        assert_eq!((x, y, ww), (0, px(46), w), "{w}x{h}");
        assert_eq!(y + wh, h - px(64) - px(16), "{w}x{h}");
        // Green takes full screen: from the foot of the bar down to the
        // bottom edge, over the dock's band, the full width.
        let (x, y, ww, wh) = crate::runner::chrome::full_screen(w, h);
        assert_eq!((x, y, ww), (0, px(46), w), "{w}x{h}");
        assert_eq!(y + wh, h, "{w}x{h}: down to the bottom edge");
        // A dock repaint commits its panel, its shadow round it and the rows
        // under it, so a dock hidden over a full-screen window leaves none.
        use crate::render::layout::{bottom_dock_rect, dock_area_rect};
        let (d, a) = (bottom_dock_rect(w, h), dock_area_rect(w, h));
        assert!(a.x + px(3) <= d.x && a.y + px(3) <= d.y, "{w}x{h}");
        assert!(a.x + a.width >= d.x + d.width + px(3), "{w}x{h}");
        assert_eq!(a.y + a.height, h, "{w}x{h}");
    }
    set_scale(4);
}

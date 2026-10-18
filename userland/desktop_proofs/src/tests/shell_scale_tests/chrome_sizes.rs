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

//! The app runtime's bar, dock, work area and full screen at the shell's
//! scale, the same numbers the shell draws with.

use super::latch::SCALE_LATCH;
use crate::shell_scale::quarters_for;
use crate::ui_font::{px, set_scale};

#[test]
fn the_runtimes_chrome_is_the_shells_at_every_scale() {
    let _held = SCALE_LATCH.lock().unwrap_or_else(|e| e.into_inner());
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

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

//! Green fills the screen at every scale, and green again restores.

use super::super::super::chrome::full_screen;
use super::super::super::desk::Desk;
use super::rig::{APP, SIZES};
use crate::window::full_screen::covers_dock_at;

/// Green fills the display from the foot of the menubar to the bottom edge,
/// full width, at every scale: the window's pixels are on the bottom row and
/// a press there goes to it.
#[test]
fn green_reaches_the_bottom_edge_at_every_scale() {
    for (w, h) in SIZES {
        let mut desk = Desk::new(w, h);
        desk.open(APP, (200, 150, 640, 480), 0);
        desk.green(APP);
        let a = desk.app(APP);
        let (_, bar, _, _) = full_screen(w, h);
        assert_eq!((a.x, a.y, a.w, a.y + a.h), (0, bar, w, h), "{w}x{h}");
        for x in [0, w / 2, w - 1] {
            assert_eq!(desk.shown_at(x, h - 1), APP, "{w}x{h}: bottom row at {x}");
            assert_eq!(desk.hit(x, h - 1), APP, "{w}x{h}: a press at the bottom");
        }
        assert!(covers_dock_at(&desk.windows, APP, 0x5749_4E00 | APP), "{w}x{h}");
        desk.assert_consistent("full screen");
    }
}

/// Green again puts back the rect the window had, and it covers the dock's
/// band no longer.
#[test]
fn green_again_restores_the_rect_the_window_had() {
    let mut desk = Desk::new(1920, 1080);
    desk.open(APP, (200, 150, 640, 480), 0);
    let before = {
        let a = desk.app(APP);
        (a.x, a.y, a.w, a.h)
    };
    desk.green(APP);
    desk.green(APP);
    let a = desk.app(APP);
    assert_eq!((a.x, a.y, a.w, a.h), before);
    assert!(!covers_dock_at(&desk.windows, APP, 0x5749_4E00 | APP));
    desk.assert_consistent("restored");
}

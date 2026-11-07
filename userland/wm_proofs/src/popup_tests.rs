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
//! Subscribers hear that focus moved whenever it did, by whatever path:
//! A popup placed against its window (window/popup.rs open) is kept below
//! the menubar's foot: the shell's chrome is drawn over every window, so a
//! menu flipped or slid up over the menubar was drawn under it while its
//! presses still went to the menu.

use crate::geometry::positioner::{Positioner, FLIP_Y, SLIDE_Y};
use crate::geometry::Rect;
use crate::window::popup::{open, PopupAsk};
use crate::window::{Kind, Visibility, Window};

const DISPLAY: (u32, u32) = (1920, 1080);
const BAR: u32 = 58;

/// A window at the top of the work area, under the menubar.
fn parent() -> Window {
    Window {
        owner_pid: 7,
        window_id: 1,
        rect: Rect { x: 200, y: BAR, width: 800, height: 600 },
        kind: Kind::Normal,
        visibility: Visibility::Visible,
        z: 1,
        in_use: true,
        full_screen: false,
        parent: 0,
        grab: false,
    }
}

/// A 200 by 300 menu anchored on (100, `ay`) of the window, growing up from
/// it (anchor top, gravity top), with `adjust`.
fn ask(ay: i32, adjust: u32) -> PopupAsk {
    let positioner = Positioner {
        anchor_x: 100,
        anchor_y: ay,
        anchor_w: 1,
        anchor_h: 1,
        width: 200,
        height: 300,
        anchor: 1,
        gravity: 1,
        adjust,
        offset_x: 0,
        offset_y: 0,
    };
    PopupAsk { window_id: 2, parent_id: 1, positioner, grab: true }
}

#[test]
fn a_menu_that_would_rise_over_the_menubar_is_flipped_below_it() {
    // Rising from 250 down the window it would start at 8, on the screen
    // but under the menubar.
    let (w, placed) = open(&ask(250, FLIP_Y), &parent(), 2, DISPLAY, BAR);
    assert!(w.rect.y >= BAR, "under the chrome: {:?}", w.rect);
    assert_eq!(w.rect.y, BAR + 251, "flipped down from its anchor");
    assert_eq!(placed.rect, w.rect);
}

#[test]
fn a_menu_slid_up_stops_at_the_menubars_foot() {
    let (w, _) = open(&ask(250, SLIDE_Y), &parent(), 2, DISPLAY, BAR);
    assert_eq!(w.rect.y, BAR, "slid to the foot, not left under the menubar");
}

#[test]
fn the_shells_own_popups_may_reach_the_top() {
    let (w, _) = open(&ask(250, SLIDE_Y), &parent(), 2, DISPLAY, 0);
    assert_eq!(w.rect.y, 8, "the shell's chrome is its own");
    let (w, _) = open(&ask(40, SLIDE_Y), &parent(), 2, (100, 20), 5000);
    assert!(w.rect.y <= 20, "a foot past the display is the display's");
}

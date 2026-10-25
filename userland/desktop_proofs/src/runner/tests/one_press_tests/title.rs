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

//! One press on a title bar raises, focuses and drags.

use super::rig::{desk, TERMINAL};

#[test]
fn one_press_on_the_title_bar_raises_focuses_and_starts_the_drag() {
    let mut d = desk();
    d.press(200, 85);
    assert_eq!(d.focused(), TERMINAL, "the press focused the terminal");
    assert_eq!(d.shown_at(400, 300), TERMINAL, "the terminal is drawn over the editor");
    assert_eq!(d.hit(400, 300), TERMINAL, "and a press there goes to it");
    assert!(d.app(TERMINAL).drag.active, "the same press started the drag");
    d.assert_consistent("after the press");

    // Straight down by 150: the window follows the pointer one for one.
    for step in 1..=15u32 {
        d.motion(200, 85 + 10 * step);
    }
    assert_eq!((d.app(TERMINAL).x, d.app(TERMINAL).y), (60, 210));
    d.assert_consistent("dragged down");
    d.release(200, 235);
    assert!(!d.app(TERMINAL).drag.active);
    assert_eq!(d.hit(200, 235), TERMINAL);
}

/// Dragging right runs the pointer, in the coordinates of the window as it was
/// at the press, across where the terminal's tabs were. Those steps went to
/// the tabs before: the window stopped following, and the release, landing
/// there too, never ended the drag.
#[test]
fn a_drag_across_the_tabs_keeps_moving_and_the_release_there_ends_it() {
    let mut d = desk();
    d.press(200, 85);
    for step in 1..=30u32 {
        d.motion(200 + 10 * step, 85);
    }
    let t = d.app(TERMINAL);
    assert_eq!((t.x, t.y), (360, 60), "the window followed all three hundred pixels");
    assert_eq!(t.accessory_events, 0, "no drag step was handed to the tabs");
    d.release(500, 85);
    assert!(!d.app(TERMINAL).drag.active, "the release over the tabs ended the drag");

    // With the button up the pointer only hovers; the window stays put.
    let before = (d.app(TERMINAL).x, d.app(TERMINAL).y);
    for step in 1..=10u32 {
        d.motion(500 + 7 * step, 85 + 3 * step);
    }
    assert_eq!((d.app(TERMINAL).x, d.app(TERMINAL).y), before);
    d.assert_consistent("after the drag");
}

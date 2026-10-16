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

//! The first frame's hold: held while the scene is empty and the deadline
//! has not passed, let go by the first layer or by the deadline, and never
//! held again once let go, whatever the scene does after.

use crate::display::first_frame_rule::{held, FIRST_FRAME_MS};

#[test]
fn an_empty_scene_is_held_until_the_deadline() {
    assert!(held(Some(100), 100, 0));
    assert!(held(Some(100), 100 + FIRST_FRAME_MS - 1, 0));
    assert!(!held(Some(100), 100 + FIRST_FRAME_MS, 0), "a boot with no splash still shows");
}

#[test]
fn the_first_layer_lets_go_at_once() {
    assert!(!held(Some(100), 101, 1));
    assert!(!held(Some(100), 100, 3));
}

#[test]
fn once_let_go_nothing_holds_again() {
    assert!(!held(None, 0, 0));
    assert!(!held(None, i64::MAX, 0));
}

#[test]
fn a_clock_that_steps_back_still_holds_and_one_that_wraps_lets_go() {
    // saturating: an uptime read below `since` counts as no time passed.
    assert!(held(Some(500), 400, 0));
    assert!(!held(Some(i64::MIN), i64::MAX, 0));
}

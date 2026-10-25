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

//! The draw order is the window manager's stack. The window manager gives
//! each raised window a new, higher z and its hit test picks the highest z
//! under the pointer; the compositor is told of every raise. These pin that
//! the compositor then draws, at every point, the window the most recent raise
//! put there, within the layer's band, so what is visibly on top is what the
//! next click lands on.

use super::stacking_rig::*;
use crate::scene::SceneTable;
use crate::scene_raise::raise_by_pid;
use crate::tests::scene::frame_model::Screen;

#[test]
fn a_new_window_opens_on_top_of_its_band() {
    let mut scene = SceneTable::new();
    open(&mut scene, 1, (0, 0, 10, 10), 2);
    open(&mut scene, 2, (0, 0, 10, 10), 2);
    open(&mut scene, 3, (0, 0, 10, 10), 2);
    assert_eq!(order(&scene), vec![1, 2, 3]);
}

#[test]
fn the_last_raised_window_draws_on_top() {
    let mut scene = SceneTable::new();
    open(&mut scene, 1, (0, 0, 10, 10), 2);
    open(&mut scene, 2, (0, 0, 10, 10), 2);
    raise_by_pid(&mut scene, 1);
    assert_eq!(order(&scene), vec![2, 1]);
}

/// The case focus alone got wrong. Windows 1, 2 and 3 open in that order, then
/// the user clicks 1 and then 2. The window manager's stack is now 3, 1, 2 from
/// the bottom. Lifting only the focused window drew 1, 3, 2: where 1 and 3
/// overlap, 3 was drawn on top while every click there went to 1.
#[test]
fn three_windows_keep_the_order_of_their_raises() {
    let mut scene = SceneTable::new();
    open(&mut scene, 1, (0, 0, 30, 20), 2);
    open(&mut scene, 2, (34, 0, 30, 20), 2);
    open(&mut scene, 3, (10, 10, 30, 20), 2);
    raise_by_pid(&mut scene, 1);
    raise_by_pid(&mut scene, 2);
    assert_eq!(order(&scene), vec![3, 1, 2]);
    let screen = Screen::composed(&scene);
    assert_eq!(screen.top_at(15, 15), 1, "1 was raised after 3, so 1 shows where they overlap");
}

#[test]
fn a_move_keeps_a_windows_place_in_the_stack() {
    let mut scene = SceneTable::new();
    open(&mut scene, 1, (0, 0, 10, 10), 2);
    open(&mut scene, 2, (5, 5, 10, 10), 2);
    open(&mut scene, 1, (20, 20, 10, 10), 2);
    assert_eq!(order(&scene), vec![1, 2], "a resubmit is not a raise");
}

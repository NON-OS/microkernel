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

//! Moving, raising, opening and closing windows repaints exactly enough. The
//! screen is kept current only through the damage the real submit, raise and
//! remove steps report, drained through the real accumulator, and after every
//! step it must equal the scene composed from scratch. A pixel that differs is
//! a half-drawn region on the real screen: a copy of a moved window left
//! behind, or part of a raised window still under the one it came up over.

use super::repaint_rig::*;
use crate::damage::DamageAccumulator;
use crate::scene::SceneTable;
use crate::scene_submit::submit_layer;
use crate::tests::scene::frame_model::{layer, Screen};

#[test]
fn a_moved_window_leaves_no_copy_behind() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (4, 4, 20, 16));
    open(&mut scene, &mut acc, 2, (34, 8, 20, 24));
    let mut screen = Screen::composed(&scene);
    while acc.drain().is_some() {}
    // The client drags window 1 down and right, the way app_skeleton submits
    // each step of a title bar drag: one scene submit at the new place.
    open(&mut scene, &mut acc, 1, (12, 22, 20, 16));
    screen.repaint(&scene, &mut acc);
    assert_current(&screen, &scene, "after the move");
    assert_eq!(screen.top_at(4, 4), 0, "the old top-left corner shows the desktop again");
}

#[test]
fn a_move_reports_both_the_old_and_the_new_place() {
    let mut scene = SceneTable::new();
    let _ = submit_layer(&mut scene, layer(7, 10, 10, 8, 8, 2));
    let out = submit_layer(&mut scene, layer(7, 30, 20, 8, 8, 2)).expect("resubmit");
    let rects: Vec<_> = out.iter().flatten().map(|r| (r.x, r.y, r.width, r.height)).collect();
    assert!(rects.contains(&(30, 20, 8, 8)), "new place: {rects:?}");
    assert!(rects.contains(&(10, 10, 8, 8)), "old place: {rects:?}");
}

#[test]
fn a_resubmit_in_place_reports_one_rectangle() {
    let mut scene = SceneTable::new();
    let _ = submit_layer(&mut scene, layer(7, 10, 10, 8, 8, 2));
    let out = submit_layer(&mut scene, layer(7, 10, 10, 8, 8, 2)).expect("resubmit");
    assert_eq!(out.iter().flatten().count(), 1);
}

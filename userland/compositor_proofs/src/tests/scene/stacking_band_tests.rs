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

//! A raise stays inside its layer's band, and the stack counter keeps the
//! order when it runs out.

use super::stacking_rig::*;
use crate::scene::SceneTable;
use crate::scene_raise::raise_by_pid;
use crate::tests::scene::frame_model::{H, W};

#[test]
fn a_raise_stays_inside_its_band() {
    let mut scene = SceneTable::new();
    open(&mut scene, 10, (0, 0, W, H), 1);
    open(&mut scene, 1, (0, 0, 20, 20), 2);
    open(&mut scene, 2, (0, 0, 20, 20), 2);
    open(&mut scene, 30, (0, 0, 8, 8), 3);
    raise_by_pid(&mut scene, 10);
    raise_by_pid(&mut scene, 1);
    assert_eq!(order(&scene), vec![10, 2, 1, 30]);
}

#[test]
fn the_stack_counter_survives_running_out() {
    let mut scene = SceneTable::new();
    scene.set_next_stack(u32::MAX - 2);
    open(&mut scene, 1, (0, 0, 10, 10), 2);
    open(&mut scene, 2, (0, 0, 10, 10), 2);
    open(&mut scene, 3, (0, 0, 10, 10), 2);
    raise_by_pid(&mut scene, 1);
    raise_by_pid(&mut scene, 2);
    assert_eq!(order(&scene), vec![3, 1, 2]);
    let top = scene.layers().map(|l| l.stack).max().expect("layers");
    assert!(top < 16, "stamps were renumbered low, top is {top}");
}

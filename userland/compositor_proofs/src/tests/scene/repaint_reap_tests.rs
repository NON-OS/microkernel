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

//! A window the reaper takes leaves the screen, the reaper hands back the
//! whole layer, and no damaged rectangle reaches past the layer it names.

use super::repaint_frame::*;
use super::repaint_rig::*;
use crate::damage::{DamageAccumulator, Rect};
use crate::scene::{Layer, SceneTable};
use crate::scene_raise::raise_by_pid;
use crate::scene_submit::submit_layer;
use crate::tests::scene::frame_model::{layer, Screen};

/// A window whose owner died without removing it: its surface stops
/// attaching, and once the reaper drops it the screen must not keep showing
/// it. The reaper used to hand back only the surface handle, so nothing
/// repainted where the window had been until the next periodic full frame.
#[test]
fn a_reaped_window_does_not_stay_on_screen() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (2, 2, 40, 30));
    open(&mut scene, &mut acc, 2, (20, 14, 40, 30));
    let mut screen = Screen::composed(&scene);
    while acc.drain().is_some() {}
    assert_eq!(screen.top_at(30, 20), 2);
    // Owner 2 dies. The cursor keeps the compositor painting a small
    // rectangle far from it, frame after frame.
    let dead = [2u64];
    for _ in 0..(REAP_THRESHOLD as usize + 5) {
        acc.accumulate(Rect { x: 0, y: 44, width: 4, height: 4 });
        while let Some(r) = acc.drain() {
            composite(&mut screen, &mut scene, &mut acc, r, &dead);
        }
    }
    assert!(scene.layers().all(|l| l.owner_pid != 2), "the reaper dropped the dead layer");
    assert_current(&screen, &scene, "after the reap");
    assert_eq!(screen.top_at(30, 20), 1, "the window under it shows again");
}

#[test]
fn the_reaper_hands_back_the_whole_layer() {
    let mut scene = SceneTable::new();
    let _ = submit_layer(&mut scene, layer(9, 3, 4, 5, 6, 2));
    let mut dropped = [Layer::default(); 4];
    for _ in 0..2 {
        assert_eq!(scene.reap_unattachable(&[], 3, &mut dropped), 0);
    }
    assert_eq!(scene.reap_unattachable(&[], 3, &mut dropped), 1);
    let d = dropped[0];
    assert_eq!((d.owner_pid, d.surface_handle, d.x, d.y, d.width, d.height), (9, 9, 3, 4, 5, 6));
}

#[test]
fn a_damaged_rectangle_never_reaches_past_the_layer_it_names() {
    let mut scene = SceneTable::new();
    let _ = submit_layer(&mut scene, layer(1, 0, 0, 10, 10, 2));
    let _ = submit_layer(&mut scene, layer(2, 5, 5, 10, 10, 2));
    let r: Rect = raise_by_pid(&mut scene, 1).expect("covered");
    assert_eq!((r.x, r.y, r.width, r.height), (0, 0, 10, 10));
}

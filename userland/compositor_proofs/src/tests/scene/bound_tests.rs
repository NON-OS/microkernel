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

//! A client's damage counts only on its own layers (state/damage_bound.rs):
//! none on a window it does not own, the part on its own where it straddles,
//! one part per layer it holds, and nothing past them.

use crate::damage::Rect;
use crate::damage_bound::by_owner;
use crate::scene::SceneTable;
use crate::scene_submit::submit_layer;
use crate::tests::scene::frame_model::layer;

fn desk() -> SceneTable {
    let mut scene = SceneTable::new();
    let _ = submit_layer(&mut scene, layer(1, 0, 0, 20, 20, 2));
    let _ = submit_layer(&mut scene, layer(2, 30, 0, 20, 20, 2));
    // Owner 1's second band: a strip along the bottom.
    let _ = submit_layer(&mut scene, layer(1, 0, 40, 64, 8, 9));
    scene
}

fn parts(scene: &SceneTable, r: Rect, owner: u32) -> Vec<(u32, u32, u32, u32)> {
    let mut v: Vec<_> =
        by_owner(r, scene.layers(), owner).map(|p| (p.x, p.y, p.width, p.height)).collect();
    v.sort();
    v
}

#[test]
fn damage_on_a_window_it_does_not_own_counts_for_nothing() {
    let scene = desk();
    assert!(parts(&scene, Rect { x: 32, y: 2, width: 10, height: 10 }, 1).is_empty());
    assert!(parts(&scene, Rect { x: 0, y: 0, width: 64, height: 48 }, 7).is_empty(), "no layers");
}

#[test]
fn damage_straddling_two_windows_keeps_the_senders_part() {
    let scene = desk();
    let r = Rect { x: 10, y: 5, width: 30, height: 4 };
    assert_eq!(parts(&scene, r, 1), [(10, 5, 10, 4)]);
    assert_eq!(parts(&scene, r, 2), [(30, 5, 10, 4)]);
}

#[test]
fn the_whole_canvas_damaged_is_each_of_the_senders_layers() {
    let scene = desk();
    let r = Rect { x: 0, y: 0, width: u32::MAX, height: u32::MAX };
    assert_eq!(parts(&scene, r, 1), [(0, 0, 20, 20), (0, 40, 64, 8)]);
}

#[test]
fn the_place_a_window_left_is_its_own_no_longer() {
    let mut scene = desk();
    let _ = submit_layer(&mut scene, layer(2, 30, 24, 20, 12, 2));
    assert!(parts(&scene, Rect { x: 30, y: 20, width: 20, height: 8 }, 2) == [(30, 24, 20, 4)]);
    assert!(parts(&scene, Rect { x: 30, y: 0, width: 20, height: 24 }, 2).is_empty());
}

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

//! Windows for the repaint proofs: opened, resized and submitted with their
//! damage kept, and the screen checked against a full frame.

use crate::damage::DamageAccumulator;
use crate::scene::{Layer, SceneTable};
use crate::scene_submit::submit_layer;
use crate::tests::scene::frame_model::{layer, Screen};

pub(crate) fn open(
    scene: &mut SceneTable,
    acc: &mut DamageAccumulator,
    owner: u32,
    r: (u32, u32, u32, u32),
) {
    let l = layer(owner, r.0, r.1, r.2, r.3, 2);
    for rect in submit_layer(scene, l).expect("room for the layer").into_iter().flatten() {
        acc.accumulate(rect);
    }
}

pub(crate) fn assert_current(screen: &Screen, scene: &SceneTable, what: &str) {
    let full = Screen::composed(scene);
    if let Some((x, y, shown, wanted)) = screen.first_difference(&full) {
        panic!("{what}: pixel ({x}, {y}) shows owner {shown}, a full frame shows {wanted}");
    }
}

/// A window on a fresh surface: `handle` is the new surface, drawn or not.
pub(crate) fn resized(owner: u32, handle: u64, r: (u32, u32, u32, u32)) -> Layer {
    Layer { surface_handle: handle, ..layer(owner, r.0, r.1, r.2, r.3, 2) }
}

pub(crate) fn submit(scene: &mut SceneTable, acc: &mut DamageAccumulator, l: Layer) {
    for rect in submit_layer(scene, l).expect("room for the layer").into_iter().flatten() {
        acc.accumulate(rect);
    }
}

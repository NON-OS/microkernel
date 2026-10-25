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

//! A desktop for the band proofs: the shell's desk and chrome bands, two
//! windows between them, and the draw order read back from the scene.

use crate::scene::SceneTable;
use crate::scene_submit::submit_layer;
use crate::tests::scene::frame_model::layer;

pub(crate) const SHELL: u32 = 7;
pub(crate) const DESK_Z: u32 = 1;
pub(crate) const APP_Z: u32 = 2;
pub(crate) const SHELL_TOP_Z: u32 = 3_000_000;

/// scene/layer.rs MAX_LAYERS_PER_OWNER.
pub(crate) const MAX_LAYERS_PER_OWNER: u32 = 4;

pub(crate) fn order(scene: &SceneTable) -> Vec<(u32, u32)> {
    let (layers, n) = scene.z_sorted_snapshot();
    layers[..n].iter().map(|l| (l.owner_pid, l.z)).collect()
}

pub(crate) fn desktop_with_two_windows() -> SceneTable {
    let mut scene = SceneTable::new();
    submit_layer(&mut scene, layer(SHELL, 0, 0, 100, 100, DESK_Z)).expect("desk");
    submit_layer(&mut scene, layer(SHELL, 0, 0, 100, 100, SHELL_TOP_Z)).expect("chrome");
    submit_layer(&mut scene, layer(1, 10, 10, 50, 50, APP_Z)).expect("window 1");
    submit_layer(&mut scene, layer(2, 20, 20, 50, 50, APP_Z)).expect("window 2");
    scene
}

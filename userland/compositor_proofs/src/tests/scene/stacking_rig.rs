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

//! Windows for the stacking proofs: one opened in a band, and the draw
//! order read back as owners.

use crate::scene::SceneTable;
use crate::scene_submit::submit_layer;
use crate::tests::scene::frame_model::layer;

/// A window as the window manager would hold it: owner, rect, z.
pub(crate) type WmEntry = (u32, (u32, u32, u32, u32), u32);

pub(crate) fn open(scene: &mut SceneTable, owner: u32, r: (u32, u32, u32, u32), band: u32) {
    submit_layer(scene, layer(owner, r.0, r.1, r.2, r.3, band)).expect("room for the layer");
}

pub(crate) fn order(scene: &SceneTable) -> Vec<u32> {
    let (layers, n) = scene.z_sorted_snapshot();
    layers[..n].iter().map(|l| l.owner_pid).collect()
}

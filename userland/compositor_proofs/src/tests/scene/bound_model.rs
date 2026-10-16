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

//! The screen for the bounded-damage proofs: each pixel the owner on top
//! there and that owner's frame count, so new content a window drew and an
//! old copy of it differ; and DAMAGE_COMMIT's handler as it takes a rectangle.

use crate::damage::{DamageAccumulator, Rect};
use crate::damage_bound::by_owner;
use crate::scene::SceneTable;
use crate::tests::scene::frame_model::{H, W};

pub fn compose(scene: &SceneTable, frames: &[u32], px: &mut [u32], clip: Rect) {
    let (layers, n) = scene.z_sorted_snapshot();
    for y in clip.y..clip.bottom().min(H) {
        for x in clip.x..clip.right().min(W) {
            let top = layers[..n]
                .iter()
                .rev()
                .find(|l| x >= l.x && x - l.x < l.width && y >= l.y && y - l.y < l.height);
            px[(y * W + x) as usize] =
                top.map_or(0, |l| l.owner_pid << 16 | frames[l.owner_pid as usize]);
        }
    }
}

/// DAMAGE_COMMIT's handler: clipped to the canvas, then bounded.
pub fn commit(scene: &SceneTable, acc: &mut DamageAccumulator, r: Rect, owner: u32) {
    if r.is_empty() || r.x >= W || r.y >= H {
        return;
    }
    let r = Rect { width: r.width.min(W - r.x), height: r.height.min(H - r.y), ..r };
    for part in by_owner(r, scene.layers(), owner) {
        acc.accumulate(part);
    }
}

/// The whole scene composed from scratch: what a full frame shows.
pub fn full(scene: &SceneTable, frames: &[u32]) -> Vec<u32> {
    let mut px = vec![0u32; (W * H) as usize];
    compose(scene, frames, &mut px, Rect { x: 0, y: 0, width: W, height: H });
    px
}

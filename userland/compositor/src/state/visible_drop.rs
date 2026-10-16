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

//! The layers a composite lets go: those whose surface is gone, and those
//! that have not attached for `REAP_THRESHOLD` paints. Each one's rectangle
//! is damaged, since its last pixels are still on screen there.

use super::attach::{AttachCache, SurfaceKernel, MAX_ATTACH};
use super::damage::{DamageAccumulator, Rect};
use super::scene::{Layer, SceneTable};
use super::scene_submit::rect_of;
use super::visible::REAP_THRESHOLD;

pub fn drop_lost(
    scene: &mut SceneTable,
    attach: &mut AttachCache,
    kernel: &mut impl SurfaceKernel,
    damage: &mut DamageAccumulator,
    gone: &[u64],
    attached: &[u64],
) {
    let mut dropped = [Layer::default(); MAX_ATTACH];
    let mut n_dropped = 0;
    for handle in gone {
        n_dropped += scene.drop_surface(*handle, &mut dropped[n_dropped..]);
    }
    n_dropped += scene.reap_unattachable(attached, REAP_THRESHOLD, &mut dropped[n_dropped..]);
    for layer in dropped.iter().take(n_dropped) {
        let _ = attach.forget(layer.surface_handle, kernel);
        // The tick's loop drains this rectangle in the same pass, so the
        // pixels go this tick.
        damage.accumulate(rect_of(layer));
    }
}

/// The rectangles of layers that have not attached, which the idle heal
/// damages (server/runner/entry.rs): nothing else on an idle desktop.
pub fn missed(scene: &SceneTable) -> impl Iterator<Item = Rect> + '_ {
    scene.layers().filter(|l| l.miss_count > 0).map(rect_of)
}

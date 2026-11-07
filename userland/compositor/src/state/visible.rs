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

//! Which layers one composite draws, and from what memory.

use super::attach::{AttachCache, Attached, SurfaceKernel, MAX_ATTACH};
use super::damage::DamageAccumulator;
use super::scene::{Layer, SceneTable};
use super::scene_submit::rect_of;
use crate::sw_blitter::Surface;

/// Paints in a row a surface may fail to attach before its layer is dropped.
pub const REAP_THRESHOLD: u16 = 60;

/// One layer to draw: the layer, the surface it is drawn from, and the
/// surface drawn over that when its content attached.
pub type Drawn = (Layer, Surface, Option<Surface>);

/// Fill `out` with the layers to draw, bottom to top, each with the surface
/// it is drawn from and its content's, and return how many. Layers whose
/// surface is gone or never attached are let go (`visible_drop`); one that
/// attaches after missing is damaged whole. A content whose surface is gone
/// is let go and its layer drawn without it, damaged whole.
pub fn visible_layers(
    scene: &mut SceneTable,
    attach: &mut AttachCache,
    kernel: &mut impl SurfaceKernel,
    damage: &mut DamageAccumulator,
    out: &mut [Drawn; MAX_ATTACH],
) -> usize {
    let (layers, count) = scene.z_sorted_snapshot();
    // Every surface the scene draws from: each layer's own and its content's.
    let mut in_scene = [0u64; 2 * MAX_ATTACH];
    for (i, l) in layers.iter().take(count.min(MAX_ATTACH)).enumerate() {
        (in_scene[2 * i], in_scene[2 * i + 1]) = (l.surface_handle, l.content.handle);
    }
    let in_scene = &in_scene[..2 * count.min(MAX_ATTACH)];
    let mut lost = [0u64; MAX_ATTACH];
    let mut n_lost = 0;
    // Handles that are not misses for the reaper: drawn, or left unmapped
    // only because every slot serves a layer (which a scene of at most
    // MAX_ATTACH layers never asks for).
    let mut attached = [0u64; MAX_ATTACH];
    let mut gone = [0u64; MAX_ATTACH];
    let (mut n, mut n_attached, mut n_gone) = (0, 0, 0);
    for layer in layers.iter().take(count) {
        match attach.lookup(layer.surface_handle, in_scene, kernel) {
            Attached::Live(src) if n < out.len() => {
                // Drawn for the first time after a miss: all of it, not only
                // where this frame's damage happens to fall.
                if layer.miss_count > 0 {
                    damage.accumulate(rect_of(layer));
                }
                let content = match attach.lookup(layer.content.handle, in_scene, kernel) {
                    Attached::Live(c) => Some(c),
                    Attached::Gone if n_lost < lost.len() => {
                        lost[n_lost] = layer.content.handle;
                        n_lost += 1;
                        None
                    }
                    _ => None,
                };
                (out[n], attached[n_attached]) = ((*layer, src, content), layer.surface_handle);
                (n, n_attached) = (n + 1, n_attached + 1);
            }
            Attached::NoRoom if n_attached < attached.len() => {
                attached[n_attached] = layer.surface_handle;
                n_attached += 1;
            }
            Attached::Gone if n_gone < gone.len() => {
                gone[n_gone] = layer.surface_handle;
                n_gone += 1;
            }
            _ => {}
        }
    }
    let (gone, attached) = (&gone[..n_gone], &attached[..n_attached]);
    super::visible_drop::drop_lost(scene, attach, kernel, damage, gone, attached);
    super::visible_drop::drop_contents(scene, damage, &lost[..n_lost]);
    let _ = attach.sweep_one(in_scene, kernel);
    n
}

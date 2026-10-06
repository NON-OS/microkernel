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

/// Fill `out` with the layers to draw, bottom to top, each with the surface
/// it is drawn from, and return how many. A layer whose surface is gone (its
/// owner exited holding it) is dropped from the scene at once and one that
/// has not attached for `REAP_THRESHOLD` paints is reaped; either way its
/// rectangle is damaged, since its last pixels are still on screen there.
pub fn visible_layers(
    scene: &mut SceneTable,
    attach: &mut AttachCache,
    kernel: &mut impl SurfaceKernel,
    damage: &mut DamageAccumulator,
    out: &mut [(Layer, Surface); MAX_ATTACH],
) -> usize {
    let (layers, count) = scene.z_sorted_snapshot();
    let mut in_scene = [0u64; MAX_ATTACH];
    for (h, l) in in_scene.iter_mut().zip(layers.iter().take(count)) {
        *h = l.surface_handle;
    }
    let in_scene = &in_scene[..count.min(MAX_ATTACH)];
    // Handles that are not misses for the reaper: drawn, or left unmapped
    // only because every slot serves a layer (which a scene of at most
    // MAX_ATTACH layers never asks for).
    let mut attached = [0u64; MAX_ATTACH];
    let mut gone = [0u64; MAX_ATTACH];
    let (mut n, mut n_attached, mut n_gone) = (0, 0, 0);
    for layer in layers.iter().take(count) {
        match attach.lookup(layer.surface_handle, in_scene, kernel) {
            Attached::Live(src) if n < out.len() => {
                out[n] = (*layer, src);
                n += 1;
                attached[n_attached] = layer.surface_handle;
                n_attached += 1;
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
    let mut dropped = [Layer::default(); MAX_ATTACH];
    let mut n_dropped = 0;
    for handle in gone.iter().take(n_gone) {
        n_dropped += scene.drop_surface(*handle, &mut dropped[n_dropped..]);
    }
    n_dropped +=
        scene.reap_unattachable(&attached[..n_attached], REAP_THRESHOLD, &mut dropped[n_dropped..]);
    for layer in dropped.iter().take(n_dropped) {
        let _ = attach.forget(layer.surface_handle, kernel);
        // The tick's loop drains this rectangle in the same pass, so the
        // pixels go this tick, not at the next periodic full frame.
        damage.accumulate(rect_of(layer));
    }
    n
}

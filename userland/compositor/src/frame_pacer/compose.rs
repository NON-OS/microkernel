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

//! One damaged rectangle composed onto the canvas. Holds no context, so the
//! host proofs compose real pixels with it exactly as a frame does.

use crate::state::attach::{AttachCache, SurfaceKernel, MAX_ATTACH};
use crate::state::damage::{DamageAccumulator, Rect};
use crate::state::scene::{Layer, SceneTable};
use crate::state::visible::visible_layers;
use crate::sw_blitter::{self, Surface};

pub const BACKGROUND_ARGB: u32 = 0xFF10_1620;

/// The scene and the memory it is drawn from.
pub struct Scene<'a, K: SurfaceKernel> {
    pub scene: &'a mut SceneTable,
    pub attach: &'a mut AttachCache,
    pub kernel: &'a mut K,
    pub damage: &'a mut DamageAccumulator,
}

/// Compose `rect` of `dst`: the background, every visible layer bottom to
/// top, then the arrow cursor at `cursor` when it is shown.
pub fn compose<K: SurfaceKernel>(
    dst: Surface,
    rect: Rect,
    s: Scene<'_, K>,
    cursor: Option<(u32, u32)>,
) {
    sw_blitter::fill_rect(dst, rect, BACKGROUND_ARGB);
    let mut layers = [(Layer::default(), Surface::default()); MAX_ATTACH];
    let count = visible_layers(s.scene, s.attach, s.kernel, s.damage, &mut layers);
    for (layer, src) in layers.iter().take(count) {
        sw_blitter::composite_layer(dst, *src, layer.x, layer.y, layer.width, layer.height, rect);
    }
    if let Some((x, y)) = cursor {
        crate::frame_pacer::cursor::blit(
            dst.base_va,
            dst.stride,
            dst.width,
            dst.height,
            x,
            y,
            rect,
        );
    }
}

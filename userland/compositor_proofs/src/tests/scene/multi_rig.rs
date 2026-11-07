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

//! Several windows of one owner for the multi-window proofs: keyed layers,
//! and a screen whose every pixel is a byte of the surface drawn there, so a
//! proof can say a window's picture is exactly what it was.

use crate::damage::{DamageAccumulator, Rect};
use crate::scene::{Layer, SceneTable};
use crate::scene_submit::submit_keyed_layer;
use crate::tests::scene::frame_model::{layer, H, W};

pub(crate) const APP_Z: u32 = 2;

/// Owner `owner`'s window `window` drawn from surface `handle`.
pub(crate) fn keyed(owner: u32, window: u32, handle: u64, r: (u32, u32, u32, u32)) -> Layer {
    let l = layer(owner, r.0, r.1, r.2, r.3, APP_Z);
    Layer { surface_handle: handle, window_id: window, ..l }
}

pub(crate) fn put(scene: &mut SceneTable, acc: &mut DamageAccumulator, l: Layer) {
    let done = submit_keyed_layer(scene, l, None).expect("room for the layer");
    for r in done.repaint.into_iter().flatten() {
        acc.accumulate(r);
    }
}

/// The pixel surface `handle` holds at (x, y) of its own picture.
pub(crate) fn content(handle: u64, x: u32, y: u32) -> u32 {
    (handle as u32).wrapping_mul(0x0101_0101) ^ (y << 16 | x).wrapping_mul(0x9E37_79B9)
}

pub(crate) struct Canvas {
    pub(crate) px: Vec<u32>,
    /// The surface each pixel came from, 0 for the background.
    pub(crate) from: Vec<u64>,
}

impl Canvas {
    pub(crate) fn composed(scene: &SceneTable) -> Canvas {
        let mut c = Canvas { px: vec![0; (W * H) as usize], from: vec![0; (W * H) as usize] };
        c.paint(scene, Rect { x: 0, y: 0, width: W, height: H });
        c
    }

    /// Compose `clip` bottom to top in the scene's own draw order.
    pub(crate) fn paint(&mut self, scene: &SceneTable, clip: Rect) {
        let (x1, y1) = ((clip.x + clip.width).min(W), (clip.y + clip.height).min(H));
        for y in clip.y..y1 {
            for x in clip.x..x1 {
                self.px[(y * W + x) as usize] = 0;
                self.from[(y * W + x) as usize] = 0;
            }
        }
        let (layers, n) = scene.z_sorted_snapshot();
        for l in &layers[..n] {
            for y in l.y.max(clip.y)..(l.y + l.height).min(y1) {
                for x in l.x.max(clip.x)..(l.x + l.width).min(x1) {
                    self.px[(y * W + x) as usize] = content(l.surface_handle, x - l.x, y - l.y);
                    self.from[(y * W + x) as usize] = l.surface_handle;
                }
            }
        }
    }

    pub(crate) fn repaint(&mut self, scene: &SceneTable, acc: &mut DamageAccumulator) {
        while let Some(r) = acc.drain() {
            self.paint(scene, r);
        }
    }

    pub(crate) fn at(&self, x: u32, y: u32) -> (u32, u64) {
        let i = (y * W + x) as usize;
        (self.px[i], self.from[i])
    }
}

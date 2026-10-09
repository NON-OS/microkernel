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

//! A screen in miniature for the stacking and repaint proofs. Each pixel holds
//! the owner of the layer on top there, composed from the real scene's draw
//! order the way `frame_pacer::composite::paint` composes a damaged rectangle:
//! the background first, then every layer bottom to top, clipped to the
//! rectangle. A screen kept up to date only through the damage the real
//! submit, raise and remove steps report must always equal one composed from
//! scratch; any pixel that differs is one a real frame leaves stale.

use crate::damage::{DamageAccumulator, Rect};
use crate::scene::{Layer, SceneTable};

pub const W: u32 = 64;
pub const H: u32 = 48;
pub const BACKGROUND: u32 = 0;

pub struct Screen {
    px: Vec<u32>,
}

impl Screen {
    /// The whole scene composed from scratch: what a full frame shows.
    pub fn composed(scene: &SceneTable) -> Screen {
        let mut s = Screen { px: vec![u32::MAX; (W * H) as usize] };
        s.paint(scene, Rect { x: 0, y: 0, width: W, height: H });
        s
    }

    /// Compose `clip` only, as the compositor does for one damaged rectangle.
    pub fn paint(&mut self, scene: &SceneTable, clip: Rect) {
        self.paint_except(scene, clip, &[]);
    }

    /// Compose `clip` with the layers whose surface is in `dead` left out, as
    /// the compositor leaves out a layer whose surface no longer attaches.
    pub fn paint_except(&mut self, scene: &SceneTable, clip: Rect, dead: &[u64]) {
        let (layers, n) = scene.z_sorted_snapshot();
        let live: Vec<Layer> =
            layers[..n].iter().filter(|l| !dead.contains(&l.surface_handle)).copied().collect();
        self.paint_layers(&live, clip);
    }

    /// Compose `clip` from `layers`, bottom to top, over the background.
    pub fn paint_layers(&mut self, layers: &[Layer], clip: Rect) {
        let (x1, y1) = ((clip.x + clip.width).min(W), (clip.y + clip.height).min(H));
        for y in clip.y..y1 {
            for x in clip.x..x1 {
                self.px[(y * W + x) as usize] = BACKGROUND;
            }
        }
        for l in layers {
            let lx1 = (l.x + l.width).min(x1);
            let ly1 = (l.y + l.height).min(y1);
            for y in l.y.max(clip.y)..ly1 {
                for x in l.x.max(clip.x)..lx1 {
                    self.px[(y * W + x) as usize] = l.owner_pid;
                }
            }
        }
    }

    /// Drain every damaged rectangle and compose each, as one frame does.
    pub fn repaint(&mut self, scene: &SceneTable, acc: &mut DamageAccumulator) {
        while let Some(r) = acc.drain() {
            self.paint(scene, r);
        }
    }

    pub fn top_at(&self, x: u32, y: u32) -> u32 {
        self.px[(y * W + x) as usize]
    }

    /// The first pixel where two screens differ, with both values.
    pub fn first_difference(&self, other: &Screen) -> Option<(u32, u32, u32, u32)> {
        (0..W * H)
            .find(|&i| self.px[i as usize] != other.px[i as usize])
            .map(|i| (i % W, i / W, self.px[i as usize], other.px[i as usize]))
    }
}

/// A layer as a client submits it: the stack stamp is the scene's to assign.
pub fn layer(owner: u32, x: u32, y: u32, w: u32, h: u32, z: u32) -> Layer {
    Layer {
        owner_pid: owner,
        surface_handle: owner as u64,
        x,
        y,
        width: w,
        height: h,
        z,
        stack: 0,
        in_use: true,
        miss_count: 0,
    }
}

/// A small deterministic generator, so a failing sequence replays exactly.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub fn below(&mut self, n: u32) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % n.max(1) as u64) as u32
    }
}

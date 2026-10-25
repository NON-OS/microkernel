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

//! A screen in miniature for the stacking and repaint proofs: each pixel the
//! owner on top there, composed from the real scene's draw order as
//! `frame_pacer::composite::paint` composes a rectangle. Kept only through the
//! damage the real steps report, it must equal one composed from scratch; a
//! pixel that differs is one a real frame leaves stale (`frame_check.rs`).

pub use super::frame_util::{layer, Rng};
use crate::damage::Rect;
use crate::scene::{Layer, SceneTable};

pub const W: u32 = 64;
pub const H: u32 = 48;
pub const BACKGROUND: u32 = 0;

pub struct Screen {
    pub(crate) px: Vec<u32>,
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
}

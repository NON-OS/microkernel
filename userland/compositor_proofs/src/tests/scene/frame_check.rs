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

//! The miniature screen read back: a frame's damage drained onto it, the
//! owner on top at a pixel, and the first pixel two screens differ at.

use super::frame_model::{Screen, H, W};
use crate::damage::DamageAccumulator;
use crate::scene::SceneTable;

impl Screen {
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

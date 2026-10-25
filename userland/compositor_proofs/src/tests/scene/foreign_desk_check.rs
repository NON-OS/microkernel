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

//! The desk's full frame: the screen composed afresh from what the scene
//! holds now, to compare the kept one with (`foreign_desk_assert.rs`).

use super::foreign_desk::*;
use super::foreign_kernel::*;
use crate::attach::AttachCache;
use crate::damage::{DamageAccumulator, Rect};
use crate::frame_pacer::compose::{compose, Scene};
use crate::sw_blitter::Surface;

impl Desk {
    /// The screen composed from scratch from what the scene holds now.
    pub(crate) fn reference(&mut self) -> Vec<u32> {
        let mut fresh = vec![0u32; (W * H) as usize];
        let dst = Surface {
            base_va: fresh.as_mut_ptr() as u64,
            stride: W * 4,
            width: W,
            height: H,
            byte_len: (W * H * 4) as u64,
        };
        let mut attach = AttachCache::new();
        let mut damage = DamageAccumulator::new();
        let s = Scene {
            scene: &mut self.scene,
            attach: &mut attach,
            kernel: &mut self.kernel,
            damage: &mut damage,
            ceiling: u32::MAX,
        };
        compose(dst, Rect { x: 0, y: 0, width: W, height: H }, s, Some(self.cursor));
        fresh
    }
}

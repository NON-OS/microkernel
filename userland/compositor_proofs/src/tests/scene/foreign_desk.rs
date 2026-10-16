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

//! A desk for the foreign-pixel proofs: the scene, the attach cache, the
//! cursor and the composed screen, in real pixels.

use super::foreign_kernel::*;
use crate::attach::AttachCache;
use crate::damage::DamageAccumulator;
use crate::scene::SceneTable;
use crate::sw_blitter::Surface;

pub(crate) struct Desk {
    pub(crate) canvas: Vec<u32>,
    pub(crate) scene: SceneTable,
    pub(crate) attach: AttachCache,
    pub(crate) kernel: Kernel,
    pub(crate) damage: DamageAccumulator,
    pub(crate) cursor: (u32, u32),
}

impl Desk {
    pub(crate) fn new() -> Desk {
        let mut d = Desk {
            canvas: vec![0; (W * H) as usize],
            scene: SceneTable::new(),
            attach: AttachCache::new(),
            kernel: Kernel::default(),
            damage: DamageAccumulator::new(),
            cursor: (0, 0),
        };
        d.kernel.register(1, W, H, W, WALL);
        d.submit(1, 1, (0, 0, W, H), 0);
        d
    }

    pub(crate) fn dst(&mut self) -> Surface {
        Surface {
            base_va: self.canvas.as_mut_ptr() as u64,
            stride: W * 4,
            width: W,
            height: H,
            byte_len: (W * H * 4) as u64,
        }
    }
}

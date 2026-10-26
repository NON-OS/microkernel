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

//! The kernel for the foreign-pixel proofs: surfaces in memory as it hands
//! them over, and a surface's frames filled with FOREIGN the moment its
//! owner exits, as another process would reuse them.

use std::collections::HashMap;

pub(crate) const W: u32 = 96;
pub(crate) const H: u32 = 64;
pub(crate) const WALL: u32 = 0xFF20_6070;
pub(crate) const WINDOW: u32 = 0xFFE0_E0E0;
pub(crate) const OTHER: u32 = 0xFF30_A040;

/// What the kernel's next owner of a freed frame put there.
pub(crate) const FOREIGN: u32 = 0xFFFF_00FF;

/// The kernel's side: each surface's memory, and which handles it still
/// knows for the compositor.
#[derive(Default)]
pub(crate) struct Kernel {
    pub(crate) memory: HashMap<u64, (Vec<u32>, u32, u32, u32)>,
    pub(crate) live: Vec<u64>,
}

impl Kernel {
    /// A surface of `w` by `h` pixels, `stride_px` pixels a row, painted
    /// `colour`; the slack past each row and past the end holds FOREIGN.
    pub(crate) fn register(&mut self, handle: u64, w: u32, h: u32, stride_px: u32, colour: u32) {
        let mut px = vec![FOREIGN; (stride_px * h + 64) as usize];
        for y in 0..h {
            for x in 0..w {
                px[(y * stride_px + x) as usize] = colour;
            }
        }
        self.memory.insert(handle, (px, w, h, stride_px));
        self.live.push(handle);
    }

    /// The owner exits: the kernel forgets the surface and its frames are
    /// somebody else's now.
    pub(crate) fn exit(&mut self, handle: u64) {
        self.live.retain(|&h| h != handle);
        if let Some((px, ..)) = self.memory.get_mut(&handle) {
            px.fill(FOREIGN);
        }
    }

    /// The handle number comes round again for a new surface in fresh
    /// memory, and the old memory stays somebody else's.
    pub(crate) fn reissue(&mut self, handle: u64, w: u32, h: u32, colour: u32) {
        let old = self.memory.remove(&handle);
        self.register(handle, w, h, w, colour);
        if let Some(old) = old {
            self.memory.insert(handle | 1 << 63, old);
        }
    }
}

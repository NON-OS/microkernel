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

//! Surfaces in host memory, attached as the kernel maps a client's shared
//! surface into the compositor.

use std::collections::HashMap;

use crate::attach::SurfaceKernel;
use crate::sw_blitter::Surface;

#[derive(Default)]
pub struct MemKernel {
    surfaces: HashMap<u64, (Vec<u32>, u32, u32)>,
}

impl MemKernel {
    /// A `w` by `h` surface whose pixels `paint` gives.
    pub fn register(&mut self, handle: u64, w: u32, h: u32, paint: impl Fn(u32, u32) -> u32) {
        let mut px = Vec::with_capacity((w * h) as usize);
        for y in 0..h {
            px.extend((0..w).map(|x| paint(x, y)));
        }
        self.surfaces.insert(handle, (px, w, h));
    }

    pub fn pixels(&mut self, handle: u64) -> Option<&mut Vec<u32>> {
        self.surfaces.get_mut(&handle).map(|s| &mut s.0)
    }
}

impl SurfaceKernel for MemKernel {
    fn attach(&mut self, handle: u64) -> Option<Surface> {
        let (px, w, h) = self.surfaces.get_mut(&handle)?;
        Some(Surface {
            base_va: px.as_mut_ptr() as u64,
            stride: *w * 4,
            width: *w,
            height: *h,
            byte_len: px.len() as u64 * 4,
        })
    }

    fn release(&mut self, handle: u64) -> bool {
        self.surfaces.contains_key(&handle)
    }
}

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

//! Surfaces in plain memory, handed to the compositor's code the way the
//! kernel hands them over: a handle maps to its pixels until dropped.

use std::collections::HashMap;

use crate::attach::SurfaceKernel;
use crate::sw_blitter::Surface;

#[derive(Default)]
pub struct Mem(pub HashMap<u64, (Vec<u32>, u32, u32)>);

impl SurfaceKernel for Mem {
    fn attach(&mut self, handle: u64) -> Option<Surface> {
        let (px, w, h) = self.0.get(&handle)?;
        let (base_va, byte_len) = (px.as_ptr() as u64, px.len() as u64 * 4);
        Some(Surface { base_va, stride: w * 4, width: *w, height: *h, byte_len })
    }
    fn release(&mut self, _: u64) -> bool {
        true
    }
}

pub fn surface(px: &mut [u32], w: u32, h: u32) -> Surface {
    let (base_va, byte_len) = (px.as_mut_ptr() as u64, px.len() as u64 * 4);
    Surface { base_va, stride: w * 4, width: w, height: h, byte_len }
}

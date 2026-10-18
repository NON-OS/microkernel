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

//! The attach cache's view of that kernel: a surface it still knows, mapped,
//! and a release it always takes.

use super::foreign_kernel::*;
use crate::attach::SurfaceKernel;
use crate::sw_blitter::Surface;

impl SurfaceKernel for Kernel {
    fn attach(&mut self, handle: u64) -> Option<Surface> {
        if !self.live.contains(&handle) {
            return None;
        }
        let (px, w, h, stride_px) = self.memory.get(&handle)?;
        Some(Surface {
            base_va: px.as_ptr() as u64,
            stride: stride_px * 4,
            width: *w,
            height: *h,
            byte_len: (*stride_px as u64) * (*h as u64) * 4,
        })
    }

    fn release(&mut self, _handle: u64) -> bool {
        true
    }
}

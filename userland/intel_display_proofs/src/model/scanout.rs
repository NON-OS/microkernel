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

//! What the engine shows: the pixel a lit plane scans out at (x, y), from
//! its latched registers, through the GGTT, out of IOVA memory.

use super::gen9::Gen9;
use crate::engine::access::field;
use crate::engine::pipe::Pipe;
use crate::engine::plane_regs as r;

impl Gen9 {
    pub fn live_reg(&self, off: u32) -> u32 {
        *self.live.get(&off).unwrap_or(&0)
    }

    /// The pixel at (x, y) of plane 1 on `p`, or None when the plane is
    /// dark, the point is outside it, or the GGTT entry is not present.
    pub fn pixel(&self, p: Pipe, x: u32, y: u32) -> Option<u32> {
        if self.live_reg(r::plane_ctl(p, 1)) & (1 << 31) == 0 {
            return None;
        }
        let size = self.live_reg(r::plane_size(p, 1));
        if x > field(size, 12, 0) || y > field(size, 27, 16) {
            return None;
        }
        let stride = field(self.live_reg(r::plane_stride(p, 1)), 9, 0) as u64 * 64;
        let gfx =
            (self.live_reg(r::plane_surf(p, 1)) & !0xFFF) as u64 + y as u64 * stride + x as u64 * 4;
        let pte = *self.ggtt.get((gfx / 4096) as usize)?;
        if pte & 1 == 0 {
            return None;
        }
        let page = self.memory.get(&(pte & 0x7F_FFFF_F000))?;
        let at = (gfx % 4096) as usize;
        Some(u32::from_le_bytes(page[at..at + 4].try_into().ok()?))
    }

    /// Fill IOVA pages `first..first+n` with one pixel value.
    pub fn fill(&mut self, first: u64, n: u64, px: u32) {
        for i in 0..n {
            let page: Vec<u8> = px.to_le_bytes().iter().copied().cycle().take(4096).collect();
            self.memory.insert(first + i * 4096, page);
        }
    }
}

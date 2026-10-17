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
//! What the watch looks through: every IOVA page an enabled plane
//! or cursor reaches, from its armed registers and its latched ones, read
//! through the GGTT as the engine would fetch them.

use super::gen9::Gen9;
use crate::engine::pipe::Pipe;
use crate::engine::plane_regs as r;

impl Gen9 {
    /// Every IOVA page an enabled plane or cursor reaches, armed or live.
    pub fn reaching(&self) -> Vec<u64> {
        let mut out = Vec::new();
        for p in Pipe::ALL {
            for live in [false, true] {
                let g = |off: u32| if live { self.live_reg(off) } else { self.get(off) };
                if g(r::plane_ctl(p, 1)) & (1 << 31) != 0 {
                    let (size, st) = (g(r::plane_size(p, 1)), g(r::plane_stride(p, 1)));
                    let bytes = ((st & 0x7FF) as u64 * 64) * (((size >> 16) & 0x1FFF) as u64 + 1);
                    self.pages(g(r::plane_surf(p, 1)), bytes.max(4096), &mut out);
                }
                if g(r::cur_ctl(p)) & 0x27 != 0 {
                    self.pages(g(r::cur_base(p)), 64 * 64 * 4, &mut out);
                }
            }
        }
        out
    }

    fn pages(&self, surf: u32, bytes: u64, out: &mut Vec<u64>) {
        let first = (surf & !0xFFF) as u64 / 4096;
        for i in 0..bytes.div_ceil(4096) {
            match self.ggtt.get((first + i) as usize) {
                Some(pte) if pte & 1 != 0 => out.push(pte & 0x7F_FFFF_F000),
                _ => {}
            }
        }
    }
}

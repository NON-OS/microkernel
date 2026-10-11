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

//! Reads that pass time, and writes that arm a latch.

use super::gen9::Gen9;
use crate::engine::ggtt::GFX_FLSH_CNTL;
use crate::engine::pipe::Pipe;
use crate::engine::plane_regs as r;
use crate::engine::regs;

impl Gen9 {
    pub(super) fn on_read(&mut self, off: u32) -> u32 {
        let timed = Pipe::ALL
            .iter()
            .any(|&p| off == regs::pipe_frmcnt(p) || off == r::plane_surflive(p, 1));
        if timed && self.period != 0 {
            self.clock += 1;
            if self.clock.is_multiple_of(self.period) {
                self.vblank();
            }
        }
        self.get(off)
    }

    pub(super) fn on_write(&mut self, off: u32, v: u32) {
        self.writes.push((off, v));
        if off == GFX_FLSH_CNTL {
            self.flushes += 1;
        }
        self.regs.insert(off, v);
        for p in Pipe::ALL {
            if off == r::plane_surf(p, 1) {
                self.armed[p.index() as usize] = true;
            }
            if off == r::cur_base(p) {
                self.cursor_armed[p.index() as usize] = true;
            }
        }
    }
}

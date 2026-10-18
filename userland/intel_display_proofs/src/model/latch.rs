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

//! Double buffering and the vertical blank, as the PRM describes them:
//! plane registers armed by PLANE_SURF, cursor registers by CUR_BASE,
//! all latched together at the blank's start, when PIPE_FRMCNT steps.

use super::gen9::Gen9;
use crate::engine::pipe::Pipe;
use crate::engine::plane_regs as r;
use crate::engine::regs;

fn plane_set(p: Pipe) -> [u32; 6] {
    [
        r::plane_ctl(p, 1),
        r::plane_stride(p, 1),
        r::plane_pos(p, 1),
        r::plane_size(p, 1),
        r::plane_offset(p, 1),
        r::plane_surf(p, 1),
    ]
}

impl Gen9 {
    pub fn vblank(&mut self) {
        for p in Pipe::ALL {
            let i = p.index() as usize;
            let n = self.get(regs::pipe_frmcnt(p));
            self.set(regs::pipe_frmcnt(p), n.wrapping_add(1));
            if std::mem::take(&mut self.armed[i]) {
                for off in plane_set(p) {
                    self.live.insert(off, self.get(off));
                }
                let surf = self.get(r::plane_surf(p, 1));
                self.set(r::plane_surflive(p, 1), surf);
            }
            if std::mem::take(&mut self.cursor_armed[i]) {
                for off in [r::cur_ctl(p), r::cur_pos(p), r::cur_base(p)] {
                    self.live.insert(off, self.get(off));
                }
            }
        }
    }
}

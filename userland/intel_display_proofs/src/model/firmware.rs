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

//! What firmware leaves lit, written straight into the model: transcoder
//! timings, the transcoder running, PIPESRC, and plane 1 scanning out a
//! linear ARGB surface at graphics address 0.

use super::gen9::Gen9;
use crate::engine::pipe::{Pipe, Transcoder};
use crate::engine::plane_regs as r;
use crate::engine::regs;

/// A timing as (active, front, sync, total) for each direction.
pub type Axis = (u32, u32, u32, u32);

pub const T1080: (Axis, Axis) = ((1920, 88, 44, 2200), (1080, 4, 5, 1125));

impl Gen9 {
    pub fn light(&mut self, t: Transcoder, p: Pipe, timing: (Axis, Axis), func: u32) {
        let pack = |lo: u32, hi: u32| ((hi - 1) << 16) | (lo - 1);
        let ((ha, hf, hs, ht), (va, vf, vs, vt)) = timing;
        self.set(regs::trans_htotal(t), pack(ha, ht));
        self.set(regs::trans_hblank(t), pack(ha, ht));
        self.set(regs::trans_hsync(t), pack(ha + hf, ha + hf + hs));
        self.set(regs::trans_vtotal(t), pack(va, vt));
        self.set(regs::trans_vblank(t), pack(va, vt));
        self.set(regs::trans_vsync(t), pack(va + vf, va + vf + vs));
        self.set(regs::trans_ddi_func_ctl(t), func | (1 << 31));
        self.set(regs::trans_conf(t), (1 << 31) | (1 << 30));
        self.set(regs::pipe_src(p), ((ha - 1) << 16) | (va - 1));
        self.set(r::plane_ctl(p, 1), (1 << 31) | (0x4 << 24));
        self.set(r::plane_stride(p, 1), ha * 4 / 64);
        self.set(r::plane_size(p, 1), ((va - 1) << 16) | (ha - 1));
        self.set(r::plane_surf(p, 1), 0);
        self.set(r::plane_surflive(p, 1), 0);
    }
}

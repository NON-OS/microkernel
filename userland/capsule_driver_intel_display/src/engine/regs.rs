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

//! Gen9 display register offsets in GTTMMADR, from the Skylake PRM, Volume
//! 2c (Command Reference: Registers), under each register's name. The
//! Kaby Lake, Coffee Lake and Comet Lake volumes keep the same offsets for
//! everything used here. Instances step by 0x1000 per pipe or transcoder
//! and 0x100 per plane, as each register's instance list gives them.

use super::pipe::{Pipe, Transcoder};

/// The transcoder's register block: TRANS_HTOTAL is its first register.
pub fn trans_base(t: Transcoder) -> u32 {
    match t {
        Transcoder::A => 0x6_0000,
        Transcoder::B => 0x6_1000,
        Transcoder::C => 0x6_2000,
        Transcoder::Edp => 0x6_F000,
    }
}

pub fn trans_htotal(t: Transcoder) -> u32 {
    trans_base(t)
}
pub fn trans_hblank(t: Transcoder) -> u32 {
    trans_base(t) + 0x04
}
pub fn trans_hsync(t: Transcoder) -> u32 {
    trans_base(t) + 0x08
}
pub fn trans_vtotal(t: Transcoder) -> u32 {
    trans_base(t) + 0x0C
}
pub fn trans_vblank(t: Transcoder) -> u32 {
    trans_base(t) + 0x10
}
pub fn trans_vsync(t: Transcoder) -> u32 {
    trans_base(t) + 0x14
}
/// TRANS_DDI_FUNC_CTL: which DDI, which protocol, polarities, eDP pipe.
pub fn trans_ddi_func_ctl(t: Transcoder) -> u32 {
    trans_base(t) + 0x400
}

/// TRANS_CONF (PIPECONF): bit 31 enable, bit 30 the transcoder's state.
pub fn trans_conf(t: Transcoder) -> u32 {
    match t {
        Transcoder::Edp => 0x7_F008,
        _ => 0x7_0008 + 0x1000 * (t as u32),
    }
}

/// PIPESRC: the pipe's source size, width-1 in 28:16, height-1 in 11:0.
pub fn pipe_src(p: Pipe) -> u32 {
    0x6_001C + 0x1000 * p.index()
}

/// PIPE_FRMCNT: counts frames, stepping at each vertical blank's start.
pub fn pipe_frmcnt(p: Pipe) -> u32 {
    0x7_0040 + 0x1000 * p.index()
}

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

//! Programming a transcoder's timing: the inverse of `timing::read`, the
//! same registers and fields (SKL PRM Vol 2c, TRANS_HTOTAL to
//! TRANS_VSYNC, PIPESRC). Blanking starts where the picture ends; there
//! are no borders.

use super::access::Mmio;
use super::error::Error;
use super::pipe::{Pipe, Transcoder};
use super::regs;
use nonos_edid::Detailed;

/// The fields hold 13 bits of a value less one.
const MAX: u32 = 8192;

fn pack(lo: u32, hi: u32) -> u32 {
    ((hi - 1) << 16) | (lo - 1)
}

/// Write `t` to `tr` and the pipe source size to `p`. A timing whose
/// totals pass the fields, or whose sync falls outside its blanking, is
/// refused before any write.
pub fn write(m: &mut dyn Mmio, tr: Transcoder, p: Pipe, t: &Detailed) -> Result<(), Error> {
    let (ht, vt) = (t.h_total(), t.v_total());
    let hs = t.h_active + t.h_front;
    let vs = t.v_active + t.v_front;
    let fits = ht <= MAX && vt <= MAX && t.h_active > 0 && t.v_active > 0 && !t.interlaced;
    let syncs = t.h_sync > 0 && t.v_sync > 0 && hs + t.h_sync <= ht && vs + t.v_sync <= vt;
    if !fits || !syncs {
        return Err(Error::Argument);
    }
    m.write(regs::trans_htotal(tr), pack(t.h_active, ht));
    m.write(regs::trans_hblank(tr), pack(t.h_active, ht));
    m.write(regs::trans_hsync(tr), pack(hs, hs + t.h_sync));
    m.write(regs::trans_vtotal(tr), pack(t.v_active, vt));
    m.write(regs::trans_vblank(tr), pack(t.v_active, vt));
    m.write(regs::trans_vsync(tr), pack(vs, vs + t.v_sync));
    m.write(regs::pipe_src(p), ((t.h_active - 1) << 16) | (t.v_active - 1));
    Ok(())
}

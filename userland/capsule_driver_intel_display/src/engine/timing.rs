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

//! The timing a transcoder is running, read back from its registers (SKL
//! PRM Vol 2c, TRANS_HTOTAL to TRANS_VSYNC): each holds an end or total
//! in 28:16 and a start or the active size in 12:0, all stored less one;
//! a start counts from the first active pixel, so sync starts at active
//! plus the front porch. The clock lives in the DPLL, which a takeover
//! leaves alone, so the timing comes back without it.

use super::access::{field, Mmio};
use super::pipe::Transcoder;
use super::regs;
use nonos_edid::Detailed;

fn pair(v: u32) -> (u32, u32) {
    (field(v, 12, 0) + 1, field(v, 28, 16) + 1)
}

/// The transcoder's timing; None when the registers describe no picture
/// a plane could fill (sync outside the blanking, totals under active).
pub fn read(m: &mut dyn Mmio, t: Transcoder) -> Option<Detailed> {
    let (h_active, h_total) = pair(m.read(regs::trans_htotal(t)));
    let (h_sync_start, h_sync_end) = pair(m.read(regs::trans_hsync(t)));
    let (v_active, v_total) = pair(m.read(regs::trans_vtotal(t)));
    let (v_sync_start, v_sync_end) = pair(m.read(regs::trans_vsync(t)));
    let ctl = m.read(regs::trans_ddi_func_ctl(t));
    let h_ok = h_active <= h_sync_start && h_sync_start < h_sync_end && h_sync_end <= h_total;
    let v_ok = v_active <= v_sync_start && v_sync_start < v_sync_end && v_sync_end <= v_total;
    if !h_ok || !v_ok {
        return None;
    }
    Some(Detailed {
        pixel_khz: 0,
        h_active,
        h_blank: h_total - h_active,
        h_front: h_sync_start - h_active,
        h_sync: h_sync_end - h_sync_start,
        v_active,
        v_blank: v_total - v_active,
        v_front: v_sync_start - v_active,
        v_sync: v_sync_end - v_sync_start,
        // TRANS_DDI_FUNC_CTL Sync Polarity: bit 16 HSync, bit 17 VSync,
        // each 1 for active high.
        h_positive: ctl & (1 << 16) != 0,
        v_positive: ctl & (1 << 17) != 0,
        interlaced: false,
        width_mm: 0,
        height_mm: 0,
    })
}

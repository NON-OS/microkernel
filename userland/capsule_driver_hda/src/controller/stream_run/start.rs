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
//! Starting an output stream descriptor on a buffer descriptor list.

use super::reset;
use crate::constants::{
    INTCTL, INTCTL_GIE, SDCTL2_DIR_OUT, SDCTL_IOCE, SDCTL_RUN, SDSTS_MASK, SD_BDPL, SD_BDPU,
    SD_CBL, SD_CTL, SD_FMT, SD_LVI, SD_STS,
};
use crate::controller::bdl::{build_bdl, N_PERIODS};
use crate::controller::dma_sync::flush;
use crate::controller::stream_layout::STREAM_BIDI;
use crate::controller::StreamDescriptor;
use crate::error::HdaResult;
use crate::regs::Regs;
use core::ptr::write_volatile;

pub(crate) struct StreamRun {
    pub desc: StreamDescriptor,
    pub tag: u8,
    pub format: u16,
    pub bdl_va: u64,
    pub bdl_dev: u64,
    pub sample_dev: u64,
    pub bytes: u32,
}

fn write_bdl(va: u64, ring_dev: u64) {
    let mut p = va;
    for e in build_bdl(ring_dev).iter() {
        // SAFETY: `va` is the BDL grant's (`restart`), a whole page; its
        // N_PERIODS entries of 16 bytes fill the first 64, fields aligned.
        unsafe {
            write_volatile(p as *mut u64, e.addr);
            write_volatile((p + 8) as *mut u32, e.len);
            write_volatile((p + 12) as *mut u32, e.flags);
        }
        p += 16;
    }
    flush(va, (N_PERIODS * 16) as u64);
}

pub(crate) fn run(regs: Regs, r: StreamRun) -> HdaResult<()> {
    write_bdl(r.bdl_va, r.sample_dev);
    let off = r.desc.mmio_offset;
    reset(regs, off)?;
    let dir = if r.desc.kind == STREAM_BIDI { SDCTL2_DIR_OUT } else { 0 };
    // SAFETY: the descriptor's registers and INTCTL lie inside the window (`crate::regs`).
    unsafe {
        regs.w8(off + SD_CTL + 2, (r.tag << 4) | dir);
        regs.w32(off + SD_CBL, r.bytes);
        regs.w16(off + SD_LVI, (N_PERIODS - 1) as u16);
        regs.w16(off + SD_FMT, r.format);
        regs.w32(off + SD_BDPL, r.bdl_dev as u32);
        regs.w32(off + SD_BDPU, (r.bdl_dev >> 32) as u32);
        regs.w8(off + SD_STS, SDSTS_MASK);
        let ctl = regs.r32(INTCTL) | INTCTL_GIE | (1u32 << r.desc.global_index);
        regs.w32(INTCTL, ctl);
        regs.w8(off + SD_CTL, SDCTL_RUN | SDCTL_IOCE);
    }
    Ok(())
}

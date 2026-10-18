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

//! Both rings stopped, sized, placed and started again.

use super::read_pointer::reset_read_pointer;
use super::ring_size;
use crate::constants::{
    CORBCTL, CORBCTL_RUN, CORBLBASE, CORBSIZE, CORBUBASE, CORBWP, RINGSIZE_16, RINGSIZE_2,
    RINGSIZE_256, RINTCNT, RINTCNT_ONE, RIRBCTL, RIRBCTL_DMAEN, RIRBCTL_RINTCTL, RIRBLBASE,
    RIRBSIZE, RIRBUBASE, RIRBWP, RIRBWP_RST,
};
use crate::regs::Regs;

/// What the bring-up found: the ring length in entries, and whether the read
/// pointer reset completed both halves of its handshake.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rings {
    pub entries: u16,
    pub rp_handshake: bool,
}

pub fn init(regs: Regs, corb_pa: u64, rirb_pa: u64) -> Rings {
    // SAFETY: the ring registers are global ones, inside the window (`crate::regs`).
    let (corb_reg, rirb_reg) = unsafe { (regs.r8(CORBSIZE), regs.r8(RIRBSIZE)) };
    let ((corb_code, corb_n), (rirb_code, rirb_n)) = (ring_size(corb_reg), ring_size(rirb_reg));
    // One length for both, so one mask indexes either ring.
    let (corb_code, rirb_code, entries) = if corb_n == rirb_n {
        (corb_code, rirb_code, corb_n)
    } else {
        let n = if corb_n < rirb_n { corb_n } else { rirb_n };
        let code = if n == 256 {
            RINGSIZE_256
        } else if n == 16 {
            RINGSIZE_16
        } else {
            RINGSIZE_2
        };
        (code, code, n)
    };
    // SAFETY: as above.
    unsafe {
        regs.w8(CORBCTL, 0);
        regs.w8(RIRBCTL, 0);
        regs.w8(CORBSIZE, corb_code);
        regs.w8(RIRBSIZE, rirb_code);
        regs.w32(CORBLBASE, corb_pa as u32);
        regs.w32(CORBUBASE, (corb_pa >> 32) as u32);
        regs.w32(RIRBLBASE, rirb_pa as u32);
        regs.w32(RIRBUBASE, (rirb_pa >> 32) as u32);
        regs.w16(CORBWP, 0);
    }
    let rp_handshake = reset_read_pointer(regs);
    // SAFETY: as above.
    unsafe {
        regs.w16(RIRBWP, RIRBWP_RST);
        regs.w16(RINTCNT, RINTCNT_ONE);
        regs.w8(CORBCTL, CORBCTL_RUN);
        regs.w8(RIRBCTL, RIRBCTL_DMAEN | RIRBCTL_RINTCTL);
    }
    Rings { entries, rp_handshake }
}

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

//! Bringing up the command (CORB) and response (RIRB) rings.
//!
//! Two things here follow Linux rather than a literal reading of the
//! specification. The CORB read pointer reset is a handshake (write RST, see it
//! read 1, write 0, see it read 0), but some controllers clear the bit on
//! their own and never show the 1, and Linux carries on after logging the
//! timeout (`azx_clear_corbrp`, sound/hda/hdac_controller.c); refusing the
//! controller there threw away a working ring. And the ring size is the
//! largest the controller says it supports in CORBSIZE/RIRBSIZE bits 7:4,
//! rather than 256 written blind: a controller offering only 16 entries
//! wrapped at 16 while the driver counted to 256.

use super::wait::until;
use crate::constants::{
    CORBCTL, CORBCTL_RUN, CORBLBASE, CORBRP, CORBRP_RST, CORBSIZE, CORBUBASE, CORBWP, RINGSIZE_16,
    RINGSIZE_2, RINGSIZE_256, RINGSIZE_CAP_16, RINGSIZE_CAP_2, RINGSIZE_CAP_256, RINTCNT,
    RINTCNT_ONE, RIRBCTL, RIRBCTL_DMAEN, RIRBCTL_RINTCTL, RIRBLBASE, RIRBSIZE, RIRBUBASE, RIRBWP,
    RIRBWP_RST,
};
use crate::regs::Regs;

/// Linux waits 1000 polls of 1 us for each half of the CORBRP handshake.
const CORBRP_MS: u64 = 2;

/// What the bring-up found: the ring length in entries, and whether the read
/// pointer reset completed both halves of its handshake.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Rings {
    pub entries: u16,
    pub rp_handshake: bool,
}

/// The size code and entry count for a CORBSIZE or RIRBSIZE value. A
/// controller that reports no sizes at all is given 256, the size every
/// controller Linux drives accepts.
pub const fn ring_size(reg: u8) -> (u8, u16) {
    if reg & RINGSIZE_CAP_256 != 0 || reg & 0xf0 == 0 {
        (RINGSIZE_256, 256)
    } else if reg & RINGSIZE_CAP_16 != 0 {
        (RINGSIZE_16, 16)
    } else if reg & RINGSIZE_CAP_2 != 0 {
        (RINGSIZE_2, 2)
    } else {
        (RINGSIZE_256, 256)
    }
}

pub fn init(regs: Regs, corb_pa: u64, rirb_pa: u64) -> Rings {
    let (corb_code, corb_n) = ring_size(unsafe { regs.r8(CORBSIZE) });
    let (rirb_code, rirb_n) = ring_size(unsafe { regs.r8(RIRBSIZE) });
    // One length for both, so one mask indexes either ring.
    let (corb_code, rirb_code, entries) = if corb_n == rirb_n {
        (corb_code, rirb_code, corb_n)
    } else {
        let n = if corb_n < rirb_n { corb_n } else { rirb_n };
        let code = if n == 256 { RINGSIZE_256 } else if n == 16 { RINGSIZE_16 } else { RINGSIZE_2 };
        (code, code, n)
    };
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
    unsafe {
        regs.w16(RIRBWP, RIRBWP_RST);
        regs.w16(RINTCNT, RINTCNT_ONE);
        regs.w8(CORBCTL, CORBCTL_RUN);
        regs.w8(RIRBCTL, RIRBCTL_DMAEN | RIRBCTL_RINTCTL);
    }
    Rings { entries, rp_handshake }
}

fn reset_read_pointer(regs: Regs) -> bool {
    unsafe { regs.w16(CORBRP, CORBRP_RST) };
    let seen = until(CORBRP_MS, || unsafe { regs.r16(CORBRP) } & CORBRP_RST != 0);
    unsafe { regs.w16(CORBRP, 0) };
    let cleared = until(CORBRP_MS, || unsafe { regs.r16(CORBRP) } & CORBRP_RST == 0);
    seen && cleared
}

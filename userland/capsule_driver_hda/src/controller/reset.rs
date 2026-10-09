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

//! A full controller reset, then the codecs that signalled after it.
//!
//! The driver used to leave a controller alone when CRST already read set,
//! and read STATESTS as it found it. On a laptop that is the common case and
//! the wrong one: UEFI firmware brings the controller out of reset itself,
//! enumerates the codecs and clears STATESTS behind it (the bits are
//! write-one-to-clear), and may leave its own CORB, RIRB and streams running.
//! STATESTS then reads zero with a working codec on the link, and the machine
//! was reported as having none. Linux resets on every probe
//! (`snd_hdac_bus_reset_link` with `full_reset`, sound/hda/hdac_controller.c):
//! stop the DMA engines, clear STATESTS, take CRST low and wait for it to read
//! low, hold it at least 100 us for the codec PLLs (HDA 1.0a section 5.5.1.2),
//! release it and wait for it to read high, then give the codecs the 521 us
//! the specification allows them to request a state change (section 4.3).

use super::wait::until;
use crate::clock::pause_ms;
use crate::constants::{
    CORBCTL, CORBCTL_RUN, DPLBASE, DPUBASE, GCAP, GCTL, GCTL_CRST, INTCTL, RIRBCTL, RIRBCTL_DMAEN,
    SDCTL_INT_MASK, SDCTL_RUN, SDSTS_MASK, SD_BASE, SD_CTL, SD_STRIDE, SD_STS, STATESTS,
    STATESTS_MASK,
};
use crate::controller::streams::stream_count;
use crate::error::{HdaError, HdaResult};
use crate::regs::Regs;

/// How long CRST and the ring run bits are given to move (Linux: 100 ms).
pub const RESET_MS: u64 = 100;
/// Held in reset, and given after it before STATESTS is read: both above the
/// 100 us and 521 us the specification names (Linux sleeps 0.5 to 1 ms and 1
/// to 1.2 ms).
pub const HOLD_MS: u64 = 1;
pub const SETTLE_MS: u64 = 2;
/// A link that shows no codec after the settle is watched this much longer
/// before it is believed empty. Some codecs signal late; the cost of the wait
/// falls only on a machine that has none.
pub const LATE_CODEC_MS: u64 = 20;

/// Reset the controller and the link and return the codecs that answered,
/// one bit per codec address.
pub fn reset_link(regs: Regs) -> HdaResult<u16> {
    stop_engines(regs);
    unsafe {
        if regs.r32(GCTL) & GCTL_CRST != 0 {
            regs.w16(STATESTS, STATESTS_MASK);
        }
        let g = regs.r32(GCTL);
        regs.w32(GCTL, g & !GCTL_CRST);
    }
    if !until(RESET_MS, || unsafe { regs.r32(GCTL) } & GCTL_CRST == 0) {
        return Err(HdaError::ControllerResetTimeout);
    }
    pause_ms(HOLD_MS);
    leave_reset(regs)?;
    pause_ms(SETTLE_MS);
    let mut mask = unsafe { regs.r16(STATESTS) } & STATESTS_MASK;
    if mask == 0 {
        until(LATE_CODEC_MS, || {
            mask = unsafe { regs.r16(STATESTS) } & STATESTS_MASK;
            mask != 0
        });
    }
    // The bits latch; clear them so a codec that drops off later shows.
    unsafe { regs.w16(STATESTS, STATESTS_MASK) };
    Ok(mask)
}

/// Set CRST, preserving the rest of GCTL, and wait for it to read back.
pub fn leave_reset(regs: Regs) -> HdaResult<()> {
    let gctl = unsafe { regs.r32(GCTL) };
    unsafe { regs.w32(GCTL, gctl | GCTL_CRST) };
    if until(RESET_MS, || unsafe { regs.r32(GCTL) } & GCTL_CRST != 0) {
        Ok(())
    } else {
        Err(HdaError::ControllerResetTimeout)
    }
}

/// Stop what a previous owner left running: every stream engine, both ring
/// engines, the position buffer and the interrupt enables. A reset under a
/// running bus master is undefined, and firmware that played a boot chime
/// leaves exactly that.
pub fn stop_engines(regs: Regs) {
    let gcap = unsafe { regs.r16(GCAP) };
    let n = stream_count(gcap) as u32;
    unsafe {
        regs.w32(INTCTL, 0);
        let mut i = 0u32;
        while i < n {
            let off = SD_BASE + i * SD_STRIDE;
            let ctl = regs.r8(off + SD_CTL);
            regs.w8(off + SD_CTL, ctl & !(SDCTL_RUN | SDCTL_INT_MASK));
            regs.w8(off + SD_STS, SDSTS_MASK);
            i += 1;
        }
        regs.w8(CORBCTL, 0);
        regs.w8(RIRBCTL, 0);
        regs.w32(DPLBASE, 0);
        regs.w32(DPUBASE, 0);
    }
    until(RESET_MS, || unsafe { regs.r8(RIRBCTL) } & RIRBCTL_DMAEN == 0);
    until(RESET_MS, || unsafe { regs.r8(CORBCTL) } & CORBCTL_RUN == 0);
}

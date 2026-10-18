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

//! Stopping every bus master the controller has, in this order: each stream
//! engine, the CORB and RIRB engines, the position buffer. Linux does the
//! same before a reset and before the driver lets go (`azx_stop_chip`,
//! `snd_hdac_bus_stop_cmd_io`). Each RUN bit is waited on until it reads 0,
//! by elapsed time, and one that never does is a named failure: a reset
//! under a running engine is undefined, and memory given back under one is
//! memory it may still write.

use super::reset::RESET_MS;
use super::stream_run;
use super::streams::stream_count;
use super::wait::until;
use crate::constants::{
    CORBCTL, CORBCTL_RUN, DPLBASE, DPUBASE, GCAP, INTCTL, RIRBCTL, RIRBCTL_DMAEN, SDSTS_MASK,
    SD_BASE, SD_STRIDE, SD_STS,
};
use crate::error::{HdaError, HdaResult};
use crate::regs::Regs;

/// Stop every engine and the interrupt enables. Every engine is told to stop
/// even when an earlier one would not, so one stuck engine leaves no other
/// running.
pub fn stop_engines(regs: Regs) -> HdaResult<()> {
    // SAFETY: GCAP and INTCTL lie in every HDA register window (section 3.3).
    let gcap = unsafe {
        regs.w32(INTCTL, 0);
        regs.r16(GCAP)
    };
    let mut stopped = true;
    for i in 0..stream_count(gcap) as u32 {
        let off = SD_BASE + i * SD_STRIDE;
        stopped &= stream_run::stop(regs, off);
        // SAFETY: stream_count holds i to the descriptors GCAP reports, at
        // most 30, all inside the window.
        unsafe { regs.w8(off + SD_STS, SDSTS_MASK) };
    }
    // SAFETY: the ring control and position buffer registers lie in every
    // HDA register window (section 3.3).
    unsafe {
        regs.w8(CORBCTL, 0);
        regs.w8(RIRBCTL, 0);
    }
    // SAFETY: as above.
    stopped &= until(RESET_MS, || unsafe { regs.r8(RIRBCTL) } & RIRBCTL_DMAEN == 0);
    // SAFETY: as above.
    stopped &= until(RESET_MS, || unsafe { regs.r8(CORBCTL) } & CORBCTL_RUN == 0);
    // The position buffer goes last, once no engine is seen running to write
    // it. SAFETY: as above.
    unsafe {
        regs.w32(DPLBASE, 0);
        regs.w32(DPUBASE, 0);
    }
    if stopped {
        Ok(())
    } else {
        Err(HdaError::EngineStopTimeout)
    }
}

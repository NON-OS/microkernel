// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Both rings stopped and started again in place (HDA 1.0a 3.3.18 to 3.3.28):
//! the run bits cleared and seen clear, both CORB pointers and the RIRB
//! write pointer reset, the error and overrun bits cleared, then both run
//! again. The base addresses and sizes the bring-up wrote are left as they
//! are: a controller keeps them while its engines are stopped, and only a
//! controller reset clears them.

use super::corb::reset_read_pointer;
use super::wait::until;
use crate::constants::{
    CORBCTL, CORBCTL_RUN, CORBSTS, CORBSTS_CMEI, CORBWP, RIRBCTL, RIRBCTL_DMAEN, RIRBCTL_RINTCTL,
    RIRBSTS, RIRBSTS_INTFL, RIRBSTS_OIS, RIRBWP, RIRBWP_RST,
};
use crate::regs::Regs;

/// How long a run bit is given to read back clear once written clear; the
/// specification names no time, and an engine stops between two fetches.
const STOP_MS: u64 = 10;

/// Stop both engines and start them again with the pointers at 0. False
/// when an engine would not stop: the rings are then left stopped, as
/// moving a pointer under a running engine is undefined.
pub(crate) fn restart(regs: Regs) -> bool {
    if !stop(regs) {
        return false;
    }
    // CORBWP back to 0 with CORBRP, as corb/rings.rs does at bring-up: an
    // engine started with the old write pointer would fetch every command
    // between 0 and it once more.
    // SAFETY: CORBWP is a global register, inside the window (`crate::regs`).
    unsafe { regs.w16(CORBWP, 0) };
    // The handshake's outcome is not a reason to give up: some controllers
    // clear the bit without showing it (corb/mod.rs).
    let _ = reset_read_pointer(regs);
    // SAFETY: the ring registers are global ones, inside the window (`crate::regs`).
    unsafe {
        regs.w16(RIRBWP, RIRBWP_RST);
        regs.w8(CORBSTS, CORBSTS_CMEI);
        regs.w8(RIRBSTS, RIRBSTS_INTFL | RIRBSTS_OIS);
        regs.w8(CORBCTL, CORBCTL_RUN);
        regs.w8(RIRBCTL, RIRBCTL_DMAEN | RIRBCTL_RINTCTL);
    }
    true
}

/// Clear both run bits and wait, bounded, for each to read back clear.
pub(super) fn stop(regs: Regs) -> bool {
    // SAFETY: as above.
    unsafe {
        regs.w8(CORBCTL, 0);
        regs.w8(RIRBCTL, 0);
    }
    // SAFETY: as above.
    let corb = until(STOP_MS, || unsafe { regs.r8(CORBCTL) } & CORBCTL_RUN == 0);
    // SAFETY: as above.
    let rirb = until(STOP_MS, || unsafe { regs.r8(RIRBCTL) } & RIRBCTL_DMAEN == 0);
    corb && rirb
}

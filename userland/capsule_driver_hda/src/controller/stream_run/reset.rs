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
//! Stopping a stream engine and resetting its descriptor.
//!
//! The reset is a handshake (HDA 1.0a section 3.3.35): RUN must read 0
//! before SRST is set, SRST must read 1 before it is cleared, and it must
//! read 0 again before any other register of the descriptor is written.
//! The driver used to write SRST straight over a running engine and carry on
//! whether or not either half was ever seen (Linux `snd_hdac_stream_reset`
//! waits for both).
//!
//! `off` is always a stream descriptor: every caller takes it from the
//! layout or from GCAP's count (`stop_engines`), so SD_CTL lies inside the
//! window (`crate::regs`).

use crate::constants::{SDCTL_INT_MASK, SDCTL_RUN, SDCTL_SRST, SD_CTL};
use crate::controller::wait::until;
use crate::error::{HdaError, HdaResult};
use crate::regs::Regs;

/// Linux gives each half of the reset 300 polls 3 us apart; RUN is given the
/// same to stop.
const RESET_MS: u64 = 5;

/// Clear RUN and the interrupt enables and wait for the engine to stop.
pub(crate) fn stop(regs: Regs, off: u32) -> bool {
    // SAFETY: SD_CTL of a stream descriptor, inside the window (above).
    unsafe {
        let ctl = regs.r8(off + SD_CTL);
        regs.w8(off + SD_CTL, ctl & !(SDCTL_RUN | SDCTL_INT_MASK));
    }
    // SAFETY: as above.
    until(RESET_MS, || unsafe { regs.r8(off + SD_CTL) } & SDCTL_RUN == 0)
}

pub(crate) fn reset(regs: Regs, off: u32) -> HdaResult<()> {
    stop(regs, off);
    // SAFETY: as in `stop`.
    unsafe { regs.w8(off + SD_CTL, SDCTL_SRST) };
    // SAFETY: as in `stop`.
    if !until(RESET_MS, || unsafe { regs.r8(off + SD_CTL) } & SDCTL_SRST != 0) {
        return Err(HdaError::StreamResetTimeout);
    }
    // SAFETY: as in `stop`.
    unsafe { regs.w8(off + SD_CTL, 0) };
    // SAFETY: as in `stop`.
    if !until(RESET_MS, || unsafe { regs.r8(off + SD_CTL) } & SDCTL_SRST == 0) {
        return Err(HdaError::StreamResetTimeout);
    }
    Ok(())
}

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

//! Waiting on the one command the driver issues, in slot 0, judged from the
//! port registers alone. `read` returns the port register at an offset from
//! the port base, so the host proofs run these same loops against a model of
//! a device.

use crate::constants::regs::{
    IS_ERR_MASK, IS_OFS, PORT_CI, PORT_IS, PORT_SACT, PORT_TFD, TFD_BSY, TFD_DRQ, TFD_ERR,
};
use crate::error::{AhciError, AhciResult};

/// The one command slot the driver issues, never as a queued command.
pub const SLOT0: u32 = 1;

/// What one look at the port says about the command in slot 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// Still running, or finished with no idle status posted yet.
    Pending,
    /// Finished with a clean status.
    Done,
    /// Failed, or the port shows work the driver never issued.
    Failed,
}

/// Judge one look at PxCI, PxSACT, PxIS and PxTFD.
pub fn verdict(ci: u32, sact: u32, is: u32, tfd: u32) -> Verdict {
    /*
     * A fatal error, an overflow (the device moved more than the PRD names)
     * or ERR in the task file fails the command whatever PxCI says: on an
     * error the HBA leaves the slot's bit set.
     */
    if is & (IS_ERR_MASK | IS_OFS) != 0 || tfd & TFD_ERR != 0 {
        return Verdict::Failed;
    }
    // Any bit but slot 0 names a command the driver never issued.
    if ci & !SLOT0 != 0 || sact != 0 {
        return Verdict::Failed;
    }
    if ci & SLOT0 != 0 {
        return Verdict::Pending;
    }
    /*
     * The slot is clear but the device still shows BSY or DRQ: its ending
     * status is not posted, so the data is not taken yet. A status that
     * never settles ends the wait in Timeout.
     */
    if tfd & (TFD_BSY | TFD_DRQ) != 0 {
        return Verdict::Pending;
    }
    Verdict::Done
}

/// Wait, until `expired` says the time is up, for the device to clear BSY and
/// DRQ before a command is issued. `expired` is asked once after each look
/// that finds the device busy. A port that then shows any slot issued or
/// queued holds work the driver did not give it, and nothing is issued into
/// it.
pub fn wait_ready(
    mut read: impl FnMut(u32) -> u32,
    mut expired: impl FnMut() -> bool,
) -> AhciResult<()> {
    while read(PORT_TFD) & (TFD_BSY | TFD_DRQ) != 0 {
        if expired() {
            return Err(AhciError::Timeout);
        }
        core::hint::spin_loop();
    }
    if read(PORT_CI) != 0 || read(PORT_SACT) != 0 {
        return Err(AhciError::CommandFailed);
    }
    Ok(())
}

/// Wait, until `expired` says the time is up, for the command in slot 0 to
/// finish. `expired` is asked once after each round that leaves it pending.
/// Each round reads PxCI first, then PxSACT, PxIS and PxTFD, so a clear slot
/// bit is judged against a status read after it, one the HBA posted before
/// it cleared the bit. Read the other way round, a command that failed
/// between the status read and the PxCI read would pass as done.
pub fn wait_done(
    mut read: impl FnMut(u32) -> u32,
    mut expired: impl FnMut() -> bool,
) -> AhciResult<()> {
    loop {
        let ci = read(PORT_CI);
        let sact = read(PORT_SACT);
        let is = read(PORT_IS);
        let tfd = read(PORT_TFD);
        match verdict(ci, sact, is, tfd) {
            Verdict::Done => return Ok(()),
            Verdict::Failed => return Err(AhciError::CommandFailed),
            Verdict::Pending => {}
        }
        if expired() {
            return Err(AhciError::Timeout);
        }
        core::hint::spin_loop();
    }
}

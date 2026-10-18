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

//! Freeing the device after a failure. A drive that may still be running
//! its command (`Abandoned`: it timed out, the HBA failed it, or the port
//! shows slots the driver never issued), one whose link is down
//! (`Unlinked`), or one that still shows BSY or DRQ (after an error of its
//! own, `Failed`, or before anything was issued, `Stuck`), is reset by a
//! COMRESET, which ends whatever it was doing before slot 0 and its buffer
//! take the next command.
//!
//! Command List Override is never used. It clears only the HBA's copy of
//! BSY and DRQ, so the next command would go to a drive still at work or
//! still in its reset, which drops it; the drive's ready status then ends
//! the slot as if the command were done, and a write or a flush that never
//! happened answers Ok. Linux's libahci sets CLO in ahci_kick_engine: before
//! the soft reset it sends with it (AHCI 1.3.1, 10.4.1), and after a failed
//! internal command, to make the engine forget it before error handling
//! goes on.

use super::cause::Cause;
use super::port::Port;
use crate::clock::Deadline;
use crate::constants::regs::{PORT_TFD, TFD_BSY, TFD_DRQ};
use crate::constants::timing::{RECOVER_READY_MS, RESET_WINDOW_MS};
use crate::error::AhciResult;
use crate::regs::Regs;

fn busy(regs: Regs, base: u32) -> bool {
    // SAFETY: the port's register block lies inside the mapped ABAR.
    (unsafe { regs.r32(base + PORT_TFD) }) & (TFD_BSY | TFD_DRQ) != 0
}

/// Free the device as `cause` needs. The caller has stopped the command
/// engine (ST and CR clear; PxSCTL.DET may change only with ST clear, AHCI
/// 1.3.1, 3.3.11) and left FIS receive on, so the drive's signature after
/// the reset reaches PxTFD and PxSIG.
pub(super) fn kick(port: &mut Port, regs: Regs, cause: Cause) -> AhciResult<()> {
    let settled = matches!(cause, Cause::Stuck | Cause::Failed);
    if settled && !busy(regs, port.base) {
        return Ok(());
    }
    comreset(port, regs)
}

/// COMRESET the port's link, opening ATA's window for the drive to come
/// out of it (`Port::reset_window`) from the moment the reset is sent.
pub(super) fn comreset(port: &mut Port, regs: Regs) -> AhciResult<()> {
    port.reset_window = Some(Deadline::after_ms(RESET_WINDOW_MS));
    super::link::link_up(regs, port.base, RECOVER_READY_MS)
}

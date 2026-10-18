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

use super::cause::Cause;
use super::port::Port;
use crate::constants::regs::{CMD_FRE, PORT_CMD, PORT_IS, PORT_SERR};
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Put a port back in order after a failure: clear its error state, stop
/// the engine, free the device as `cause` needs (`kick`), and restart the
/// engine. A CR that will not clear is met with a COMRESET, as AHCI 1.3.1,
/// 10.4.2 allows, and one more stop. `Wedged` when even that leaves CR
/// set: the HBA may still own the command list, so the engine is left as
/// it is. Otherwise `Err` when the device stayed stuck or the engine would
/// not start; the next command on the port then fails in its own wait,
/// never hangs. Every COMRESET is noted in the port (`kick::comreset`).
pub(super) fn recover(port: &mut Port, regs: Regs, cause: Cause) -> AhciResult<()> {
    let base = port.base;
    clear_errors(regs, base);
    let reset = match super::stop::stop_engine(regs, base) {
        Ok(()) => None,
        Err(_) => {
            let reset = super::kick::comreset(port, regs);
            super::stop::stop_engine(regs, base).map_err(|_| AhciError::Wedged)?;
            Some(reset)
        }
    };
    /*
     * ST may be set only with FRE set (AHCI 1.3.1, 3.3.7), and without it
     * the HBA posts no D2H FIS, so PxTFD would keep the failed command's
     * status.
     */
    // SAFETY: the port's register block lies inside the mapped ABAR.
    unsafe {
        regs.w32(base + PORT_CMD, regs.r32(base + PORT_CMD) | CMD_FRE);
    }
    // The COMRESET that freed CR has reset the drive too.
    let kicked = reset.unwrap_or_else(|| super::kick::kick(port, regs, cause));
    // A COMRESET latches link errors of its own.
    clear_errors(regs, base);
    /*
     * ST is set whether or not the drive's signature came in time. AHCI
     * 1.3.1, 10.3.1 asks for BSY and DRQ clear and the link up first;
     * Linux's ahci_do_hardreset restarts the engine whatever the reset
     * gave, and so does this. Nothing is issued until the drive shows idle
     * (`holdup`), and the reset window there tells a drive still resetting
     * from a stuck port only while ST and CR read set.
     */
    super::start::start(regs, base)?;
    kicked
}

/// Clear PxSERR, then PxIS: PCS and PRCS mirror PxSERR bits and clear only
/// with them.
pub(super) fn clear_errors(regs: Regs, base: u32) {
    // SAFETY: the port's register block lies inside the mapped ABAR.
    unsafe {
        regs.w32(base + PORT_SERR, regs.r32(base + PORT_SERR));
        regs.w32(base + PORT_IS, u32::MAX);
    }
}

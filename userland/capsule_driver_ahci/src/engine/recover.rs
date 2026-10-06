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

use crate::constants::regs::{CMD_FRE, PORT_CMD, PORT_IS, PORT_SERR};
use crate::error::AhciResult;
use crate::regs::Regs;

/// Put a port back in order after a failed command: clear its error state,
/// free a device stuck in BSY or DRQ (`kick`), and restart the engine. `Err`
/// when the engine would not stop or start, or the device stayed stuck; the
/// next command on the port then fails in its own wait, never hangs.
pub(super) fn recover(regs: Regs, base: u32, sclo: bool) -> AhciResult<()> {
    clear_errors(regs, base);
    super::stop::stop_engine(regs, base)?;
    /*
     * ST may be set only with FRE set (AHCI 1.3.1, 3.3.7), and without it
     * the HBA posts no D2H FIS, so PxTFD would keep the failed command's
     * status and fail the next one.
     */
    unsafe {
        regs.w32(base + PORT_CMD, regs.r32(base + PORT_CMD) | CMD_FRE);
    }
    let kicked = super::kick::kick(regs, base, sclo);
    // A COMRESET latches link errors of its own.
    clear_errors(regs, base);
    super::start::start(regs, base)?;
    kicked
}

fn clear_errors(regs: Regs, base: u32) {
    unsafe {
        regs.w32(base + PORT_SERR, regs.r32(base + PORT_SERR));
        regs.w32(base + PORT_IS, u32::MAX);
    }
}

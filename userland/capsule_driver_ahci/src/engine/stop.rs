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

use crate::clock::wait_until;
use crate::constants::regs::{CMD_CR, CMD_FR, CMD_FRE, CMD_ST, PORT_CMD};
use crate::constants::timing::ENGINE_STOP_MS;
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Stop the command list engine and FIS receive. `Err` when CR or FR did not
/// clear in time: the HBA may then still DMA into the port's command list
/// or FIS region, and the caller must not give that memory back.
pub(super) fn stop(regs: Regs, base: u32) -> AhciResult<()> {
    stop_engine(regs, base)?;
    unsafe {
        let cmd = regs.r32(base + PORT_CMD);
        regs.w32(base + PORT_CMD, cmd & !CMD_FRE);
    }
    /*
     * FR clears once the HBA stops writing received FISes; until then the
     * FIS region must stay mapped (AHCI 1.3.1, PxCMD.FR).
     */
    if !wait_until(ENGINE_STOP_MS, || unsafe { regs.r32(base + PORT_CMD) } & CMD_FR == 0) {
        return Err(AhciError::Timeout);
    }
    Ok(())
}

/// Stop the command list engine alone, leaving FIS receive on. CR clears
/// once the engine stops; FRE may be cleared only after it has (AHCI 1.3.1,
/// 10.1.2).
pub(super) fn stop_engine(regs: Regs, base: u32) -> AhciResult<()> {
    unsafe {
        let cmd = regs.r32(base + PORT_CMD);
        regs.w32(base + PORT_CMD, cmd & !CMD_ST);
    }
    if !wait_until(ENGINE_STOP_MS, || unsafe { regs.r32(base + PORT_CMD) } & CMD_CR == 0) {
        return Err(AhciError::Timeout);
    }
    Ok(())
}

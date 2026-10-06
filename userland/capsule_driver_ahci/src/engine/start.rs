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
use crate::constants::regs::{CMD_CR, CMD_ST, PORT_CMD};
use crate::constants::timing::ENGINE_STOP_MS;
use crate::error::{AhciError, AhciResult};
use crate::regs::Regs;

/// Start the command list engine. ST may be set only once CR reads clear
/// (AHCI 1.3.1, 10.3.1); a CR that never clears leaves the engine off.
pub(super) fn start(regs: Regs, base: u32) -> AhciResult<()> {
    if !wait_until(ENGINE_STOP_MS, || unsafe { regs.r32(base + PORT_CMD) } & CMD_CR == 0) {
        return Err(AhciError::Timeout);
    }
    // FRE was already enabled during program(); only ST is set here, after
    // the link is up, so command processing starts against a ready device.
    unsafe {
        let cmd = regs.r32(base + PORT_CMD);
        regs.w32(base + PORT_CMD, cmd | CMD_ST);
    }
    Ok(())
}

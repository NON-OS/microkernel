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

//! Freeing a port whose device holds BSY or DRQ. Nothing can be issued into
//! such a port, and a failed command often leaves it that way. Linux does the
//! same in ahci_kick_engine: Command List Override when the HBA has it, and a
//! COMRESET of the link otherwise (or when CLO did not free it).

use crate::clock::wait_until;
use crate::constants::regs::{CMD_CLO, PORT_CMD, PORT_TFD, TFD_BSY, TFD_DRQ};
use crate::constants::timing::{ENGINE_STOP_MS, RECOVER_READY_MS};
use crate::error::AhciResult;
use crate::regs::Regs;

fn busy(regs: Regs, base: u32) -> bool {
    (unsafe { regs.r32(base + PORT_TFD) }) & (TFD_BSY | TFD_DRQ) != 0
}

/// Clear a stuck BSY or DRQ. The caller has stopped the command engine (ST
/// and CR clear) and left FIS receive on: CLO may be set only with ST clear
/// (AHCI 1.3.1, 3.3.7), and a COMRESET only then too. `sclo` is CAP.SCLO.
pub(super) fn kick(regs: Regs, base: u32, sclo: bool) -> AhciResult<()> {
    if !busy(regs, base) {
        return Ok(());
    }
    if sclo {
        unsafe {
            regs.w32(base + PORT_CMD, regs.r32(base + PORT_CMD) | CMD_CLO);
        }
        let cleared =
            wait_until(ENGINE_STOP_MS, || unsafe { regs.r32(base + PORT_CMD) } & CMD_CLO == 0);
        if cleared && !busy(regs, base) {
            return Ok(());
        }
    }
    super::link::link_up(regs, base, RECOVER_READY_MS)
}

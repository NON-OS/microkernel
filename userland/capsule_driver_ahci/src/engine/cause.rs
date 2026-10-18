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

use super::verdict::SLOT0;
use crate::constants::regs::{IS_ERR_MASK, IS_LINK_CHANGE, IS_OFS, IS_TFES};
use crate::error::AhciError;

/// Why a port is being recovered, which decides how hard (`kick`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    /// Nothing was issued: the device held BSY or DRQ, or the engine was
    /// off.
    Stuck,
    /// The drive ended the issued command with an error of its own.
    Failed,
    /// The issued command never finished, the HBA failed it, or the port
    /// shows slots the driver never issued, before issue or after: the
    /// drive may still be running a command, and move its data when slot 0
    /// holds the next one.
    Abandoned,
    /// Nothing was issued: a device is present but its link is down
    /// (PxSSTS.DET 1), which only a COMRESET brings back.
    Unlinked,
}

impl Cause {
    /// Why the issued command in slot 0 failed with `e`, from PxIS, PxCI
    /// and PxSACT as it left them. Only an error the drive posted (TFES, or
    /// ERR once the slot cleared) says the drive is done with the command.
    /// A timeout, a fatal HBA or interface error (HBFS, HBDS, IFS), an
    /// overflow, a change of the link (PRCS, PCS), or a slot the driver
    /// never issued can leave it mid-command, or the command never run;
    /// Linux's ahci_error_intr asks for a reset after the fatal errors and
    /// the link changes too.
    pub fn of(e: AhciError, is: u32, ci: u32, sact: u32) -> Self {
        let fatal = is & (IS_ERR_MASK & !IS_TFES | IS_OFS | IS_LINK_CHANGE) != 0;
        let foreign = ci & !SLOT0 != 0 || sact != 0;
        if e == AhciError::Timeout || fatal || foreign {
            Cause::Abandoned
        } else {
            Cause::Failed
        }
    }
}

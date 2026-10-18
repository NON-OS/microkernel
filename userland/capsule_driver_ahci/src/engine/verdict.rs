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

//! What one look at the port says about the command in slot 0.

use crate::constants::regs::{IS_ERR_MASK, IS_LINK_CHANGE, IS_OFS, TFD_BSY, TFD_DRQ, TFD_ERR};

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
     * A fatal error, an overflow (the device moved more than the PRD
     * names), or a change of the link or the device on it fails the
     * command whatever PxCI says: on an error the HBA sets PxIS.TFES and
     * leaves the slot's bit set (AHCI 1.3.1, 6.2.2), and a drive that
     * reset itself dropped the command, while the signature it posts after
     * clears the slot as if the command were done.
     */
    if is & (IS_ERR_MASK | IS_OFS | IS_LINK_CHANGE) != 0 {
        return Verdict::Failed;
    }
    // Any bit but slot 0 names a command the driver never issued.
    if ci & !SLOT0 != 0 || sact != 0 {
        return Verdict::Failed;
    }
    /*
     * PxTFD changes only when a FIS arrives (AHCI 1.3.1, 3.3.8), so while
     * the slot is outstanding it may still hold the last command's status,
     * ERR and all: an error in this one shows in PxIS, which was cleared
     * before the issue. Once the HBA has cleared the slot bit it has
     * posted this command's ending status (5.6.2), and only then is ERR
     * this command's.
     */
    if ci & SLOT0 != 0 {
        return Verdict::Pending;
    }
    if tfd & TFD_ERR != 0 {
        return Verdict::Failed;
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

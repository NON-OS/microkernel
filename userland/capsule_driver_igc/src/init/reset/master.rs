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

//! igc_disable_pcie_master: set CTRL.GIO_MASTER_DISABLE and wait for
//! STATUS.GIO_MASTER_ENABLE to drop, so no request is outstanding on the
//! link when the MAC resets. Linux logs a timeout here and resets anyway;
//! this driver stops the attempt instead, and the next attempt starts from
//! a fresh claim, because a reset under a live master cycle is the PCIe
//! hang the wait exists to avoid.

use crate::constants::ctrl::{CTRL_GIO_MASTER_DISABLE, STATUS_GIO_MASTER_ENABLE};
use crate::constants::regs::{REG_CTRL, REG_STATUS};
use crate::constants::timeouts::MASTER_DISABLE_MS;
use crate::init::wait::until;
use crate::regs::Regs;

/// Quotes MASTER_DISABLE_MS; igc_proofs holds the two equal.
pub const MASTER_STUCK: &str = "PCIe master did not stop in 2400 ms";

pub fn stop(regs: &Regs) -> Result<(), &'static str> {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and both
    // offsets are 32-bit registers inside it (igc_regs.h).
    unsafe {
        let ctrl = regs.r32(REG_CTRL);
        regs.w32(REG_CTRL, ctrl | CTRL_GIO_MASTER_DISABLE);
    }
    // SAFETY: as above.
    let idle = || unsafe { regs.r32(REG_STATUS) } & STATUS_GIO_MASTER_ENABLE == 0;
    if until(MASTER_DISABLE_MS, idle) {
        Ok(())
    } else {
        Err(MASTER_STUCK)
    }
}

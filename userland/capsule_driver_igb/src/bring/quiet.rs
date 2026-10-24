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

//! Quieting the part before its reset. Firmware (PXE, UEFI UNDI) or a
//! previous OS may have left queues running, and a reset that lands on a
//! bus-master cycle still in flight can hang the PCIe link. So bus
//! mastering is asked to stop and seen stopped first; if it never stops
//! the attempt ends here, and the next one starts from a fresh claim. Then
//! every cause is masked, both engines stopped, the posted writes flushed
//! with a read, and the part left a moment to settle.

use crate::bring::wait::{hold, until};
use crate::facts::bounds::{MASTER_DISABLE_MS, QUIESCE_SETTLE_MS};
use crate::facts::mac::{CTRL_GIO_MASTER_DISABLE, STATUS_GIO_MASTER_ENABLE};
use crate::facts::regs::{REG_CTRL, REG_IMC, REG_RCTL, REG_STATUS, REG_TCTL};
use crate::facts::tx::TCTL_PSP;
use crate::regs::Regs;

/// Quotes MASTER_DISABLE_MS; igb_proofs holds the two equal.
pub const MASTER_STUCK: &str = "PCIe master did not stop in 80 ms";

/*
 * SAFETY for every unsafe in this file: `regs` carries a valid broker MmioMap
 * base for BAR0 and every offset is a 32-bit register inside it (facts::regs).
 */

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    unsafe {
        let ctrl = regs.r32(REG_CTRL);
        regs.w32(REG_CTRL, ctrl | CTRL_GIO_MASTER_DISABLE);
    }
    let idle = || unsafe { regs.r32(REG_STATUS) } & STATUS_GIO_MASTER_ENABLE == 0;
    if !until(MASTER_DISABLE_MS, idle) {
        return Err(MASTER_STUCK);
    }
    unsafe {
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        regs.w32(REG_RCTL, 0);
        regs.w32(REG_TCTL, TCTL_PSP);
        let _ = regs.r32(REG_STATUS);
    }
    hold(QUIESCE_SETTLE_MS);
    Ok(())
}

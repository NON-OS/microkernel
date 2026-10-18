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
//! previous OS may have left it running: the receiver goes off, every cause
//! is masked and what is latched read clear, queue 0 is stopped and the
//! posted writes flushed with a read. Then bus mastering is asked to stop
//! and seen stopped, since a reset that lands on a master cycle in flight
//! can hang the PCIe link; if it never stops the attempt ends here, and the
//! next one starts from a fresh claim.

use crate::bring::wait::until;
use crate::facts::bounds::MASTER_DISABLE_MS;
use crate::facts::mac::{CTRL_GIO_DIS, RXCTRL_RXEN, STATUS_GIO};
use crate::facts::regs::{REG_CTRL, REG_EICR, REG_EIMC, REG_EIMC_EX0, REG_EIMC_EX1};
use crate::facts::regs::{REG_RXCTRL, REG_RXDCTL, REG_STATUS, REG_TXDCTL};
use crate::regs::Regs;

/// Quotes MASTER_DISABLE_MS; ixgbe_proofs holds the two equal.
pub const MASTER_STUCK: &str = "PCIe master did not stop in 80 ms";

/*
 * SAFETY for every unsafe in this file: `regs` carries a valid broker MmioMap
 * base for BAR0 and every offset is a 32-bit register inside it (facts::regs).
 */

pub fn run(regs: &Regs) -> Result<(), &'static str> {
    unsafe {
        let rxctrl = regs.r32(REG_RXCTRL);
        regs.w32(REG_RXCTRL, rxctrl & !RXCTRL_RXEN);
        mask_all(regs);
        let _ = regs.r32(REG_EICR);
        regs.w32(REG_TXDCTL, 0);
        regs.w32(REG_RXDCTL, 0);
        let _ = regs.r32(REG_STATUS);
        let ctrl = regs.r32(REG_CTRL);
        regs.w32(REG_CTRL, ctrl | CTRL_GIO_DIS);
    }
    let idle = || unsafe { regs.r32(REG_STATUS) } & STATUS_GIO == 0;
    if until(MASTER_DISABLE_MS, idle) {
        Ok(())
    } else {
        Err(MASTER_STUCK)
    }
}

/// Every cause masked: the other causes and every queue's.
///
/// # Safety
/// `regs` must carry a valid BAR0 mapping.
pub unsafe fn mask_all(regs: &Regs) {
    regs.w32(REG_EIMC, 0xFFFF_0000);
    regs.w32(REG_EIMC_EX0, 0xFFFF_FFFF);
    regs.w32(REG_EIMC_EX1, 0xFFFF_FFFF);
}

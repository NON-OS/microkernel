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

//! ixgbe_stop_adapter before the reset: receive off, every cause masked and
//! the latched ones read clear, queue 0 stopped, and the posted writes
//! flushed with a read. Firmware (PXE, UEFI UNDI) or a previous OS can leave
//! the part running.

use crate::constants::ctrl::RXCTRL_RXEN;
use crate::constants::regs::{REG_EICR, REG_EIMC, REG_EIMC_EX0, REG_EIMC_EX1, REG_RXCTRL};
use crate::constants::regs::{REG_RXDCTL, REG_STATUS, REG_TXDCTL};
use crate::regs::Regs;

pub fn run(regs: &Regs) {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (ixgbe_type.h).
    unsafe {
        let rxctrl = regs.r32(REG_RXCTRL);
        regs.w32(REG_RXCTRL, rxctrl & !RXCTRL_RXEN);
        mask_all(regs);
        let _ = regs.r32(REG_EICR);
        regs.w32(REG_TXDCTL, 0);
        regs.w32(REG_RXDCTL, 0);
        let _ = regs.r32(REG_STATUS);
    }
}

/// ixgbe_irq_disable on the 82599 and later: EIMC for the other causes,
/// EIMC_EX for every queue's.
///
/// # Safety
/// `regs` must carry a valid BAR0 mapping.
pub unsafe fn mask_all(regs: &Regs) {
    regs.w32(REG_EIMC, 0xFFFF_0000);
    regs.w32(REG_EIMC_EX0, 0xFFFF_FFFF);
    regs.w32(REG_EIMC_EX1, 0xFFFF_FFFF);
}

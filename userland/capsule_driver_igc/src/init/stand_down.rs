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

//! What a failed attempt leaves the part doing before its grants go back:
//! nothing. A step that fails after the MAC enables (a queue that did not
//! read back) would otherwise leave a queue able to DMA into ring memory the
//! broker is about to hand to someone else. Both queues are disabled, both
//! engines stopped, every cause masked, and bus mastering asked to stop.

use crate::constants::ctrl::CTRL_GIO_MASTER_DISABLE;
use crate::constants::regs::{REG_CTRL, REG_IMC, REG_RCTL, REG_RXDCTL, REG_STATUS};
use crate::constants::regs::{REG_TCTL, REG_TXDCTL};
use crate::constants::tx_bits::TCTL_PSP;
use crate::regs::Regs;

pub fn run(regs: &Regs) {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (igc_regs.h).
    unsafe {
        regs.w32(REG_RXDCTL, 0);
        regs.w32(REG_TXDCTL, 0);
        regs.w32(REG_RCTL, 0);
        regs.w32(REG_TCTL, TCTL_PSP);
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        let ctrl = regs.r32(REG_CTRL);
        regs.w32(REG_CTRL, ctrl | CTRL_GIO_MASTER_DISABLE);
        let _ = regs.r32(REG_STATUS);
    }
}

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
//! engines stopped, every cause masked, and bus mastering asked to stop,
//! then waited on, as Linux's igb_disable_pcie_master does, so no grant
//! goes back while the part may still be writing to it.

use crate::facts::mac::{CTRL_GIO_MASTER_DISABLE, STATUS_GIO_MASTER_ENABLE};
use crate::facts::regs::{REG_CTRL, REG_IMC, REG_RCTL, REG_RXDCTL, REG_STATUS};
use crate::facts::regs::{REG_TCTL, REG_TXDCTL};
use crate::facts::tx::TCTL_PSP;
use crate::regs::Regs;

use super::wait::until;

/// How long the master gets to finish (MASTER_DISABLE_TIMEOUT, 800 x 100 us).
pub const MASTER_STOP_MS: u64 = 80;

/// Whether the part's bus master stopped within MASTER_STOP_MS.
pub fn run(regs: &Regs) -> bool {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (facts::regs).
    unsafe {
        regs.w32(REG_RXDCTL, 0);
        regs.w32(REG_TXDCTL, 0);
        regs.w32(REG_RCTL, 0);
        regs.w32(REG_TCTL, TCTL_PSP);
        regs.w32(REG_IMC, 0xFFFF_FFFF);
        let ctrl = regs.r32(REG_CTRL);
        regs.w32(REG_CTRL, ctrl | CTRL_GIO_MASTER_DISABLE);
    }
    // SAFETY: as above; STATUS is a 32-bit register in BAR0.
    until(MASTER_STOP_MS, || unsafe { regs.r32(REG_STATUS) } & STATUS_GIO_MASTER_ENABLE == 0)
}

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

//! igc_setup_tctl and igc_setup_rctl, in igc_configure's order: both run
//! before the rings are programmed, each with its queue disabled first so
//! the enable cannot start a queue whose base is not yet written.
//!
//! RCTL.LPE is left clear where Linux sets it: the buffers are 2 KiB and the
//! protocol caps a frame at 1514 bytes, so a long frame is refused by the
//! MAC instead of spilling into a second descriptor.

use crate::constants::ctrl::RFCTL_IPV6_EX_DIS;
use crate::constants::regs::{REG_RCTL, REG_RFCTL, REG_RXDCTL, REG_TCTL, REG_TXDCTL};
use crate::constants::rx_bits::{
    RCTL_BAM, RCTL_EN, RCTL_LBM_MASK, RCTL_LPE, RCTL_MO_MASK, RCTL_SBP, RCTL_SECRC, RCTL_SZ_MASK,
};
use crate::constants::tx_bits::{TCTL_CT_15, TCTL_CT_MASK, TCTL_EN, TCTL_PSP, TCTL_RTLC};
use crate::regs::Regs;

pub fn enable(regs: &Regs) {
    // SAFETY: `regs` carries a valid broker MmioMap base for BAR0 and every
    // offset is a 32-bit register inside it (igc_regs.h).
    unsafe {
        regs.w32(REG_TXDCTL, 0);
        let tctl = regs.r32(REG_TCTL) & !TCTL_CT_MASK;
        regs.w32(REG_TCTL, tctl | TCTL_PSP | TCTL_RTLC | TCTL_CT_15 | TCTL_EN);

        let mut rctl = regs.r32(REG_RCTL);
        rctl &= !(RCTL_MO_MASK | RCTL_LBM_MASK | RCTL_SBP | RCTL_SZ_MASK | RCTL_LPE);
        rctl |= RCTL_EN | RCTL_BAM | RCTL_SECRC;
        regs.w32(REG_RXDCTL, 0);
        regs.w32(REG_RCTL, rctl);

        // igc_rx_fifo_flush_base: "disable IPv6 options as per hardware errata".
        let rfctl = regs.r32(REG_RFCTL);
        regs.w32(REG_RFCTL, rfctl | RFCTL_IPV6_EX_DIS);
    }
}

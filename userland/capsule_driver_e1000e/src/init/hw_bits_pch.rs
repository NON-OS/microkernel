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

//! The required hardware bits for the PCH parts: Linux
//! e1000_initialize_hw_bits_ich8lan, then the transmit write-back policy,
//! the pch_tgp DMA clock workaround and relaxed-ordering off from
//! e1000_init_hw_ich8lan. Bits Linux sets by number are set by number here.

use crate::constants::ctrl::{CTRL_EXT_HW_BIT22, CTRL_EXT_PHYPDEN, CTRL_EXT_RO_DIS};
use crate::constants::pch_bits::FFLT_DBG_DONT_GATE_WAKE_DMA_CLK;
use crate::constants::regs::{
    REG_CTRL_EXT, REG_RFCTL, REG_TARC0, REG_TARC1, REG_TCTL, REG_TXDCTL0, REG_TXDCTL1,
};
use crate::constants::regs_pch::REG_FFLT_DBG;
use crate::constants::rxtx::{
    RFCTL_NFSR_DIS, RFCTL_NFSW_DIS, TCTL_MULR, TXDCTL_COUNT_DESC, TXDCTL_FULL_TX_DESC_WB,
    TXDCTL_MAX_TX_DESC_PREFETCH, TXDCTL_PTHRESH, TXDCTL_WTHRESH,
};
use crate::constants::Family;
use crate::regs::Regs;

const TARC0_SET: u32 = (1 << 23) | (1 << 24) | (1 << 26) | (1 << 27);
const TARC1_SET: u32 = (1 << 24) | (1 << 26) | (1 << 30);
const TARC1_MULR_OFF: u32 = 1 << 28;

pub fn run(regs: &Regs, family: Family) {
    // SAFETY: `regs` is the broker-mapped BAR0 window; every offset here is
    // a 4-byte register inside it on the PCH parts.
    unsafe {
        regs.modify(REG_CTRL_EXT, 0, CTRL_EXT_HW_BIT22 | CTRL_EXT_PHYPDEN);
        for txdctl in [REG_TXDCTL0, REG_TXDCTL1] {
            regs.modify(txdctl, 0, TXDCTL_COUNT_DESC);
        }
        regs.modify(REG_TARC0, 0, TARC0_SET);
        // TARC1 bit 28 is set only while TCTL.MULR is clear.
        let mulr = regs.r32(REG_TCTL) & TCTL_MULR != 0;
        let (clear, set) = if mulr { (TARC1_MULR_OFF, 0) } else { (0, TARC1_MULR_OFF) };
        regs.modify(REG_TARC1, clear, set | TARC1_SET);
        // The NFS filter corrupts descriptors under NFSv2 UDP traffic.
        regs.modify(REG_RFCTL, 0, RFCTL_NFSW_DIS | RFCTL_NFSR_DIS);
        for txdctl in [REG_TXDCTL0, REG_TXDCTL1] {
            regs.modify(txdctl, TXDCTL_WTHRESH, TXDCTL_FULL_TX_DESC_WB);
            regs.modify(txdctl, TXDCTL_PTHRESH, TXDCTL_MAX_TX_DESC_PREFETCH);
        }
        // Packet loss on pch_tgp and later: keep the DMA clock ungated.
        if family >= Family::PchTgp {
            regs.modify(REG_FFLT_DBG, 0, FFLT_DBG_DONT_GATE_WAKE_DMA_CLK);
        }
        regs.modify(REG_CTRL_EXT, 0, CTRL_EXT_RO_DIS);
    }
}

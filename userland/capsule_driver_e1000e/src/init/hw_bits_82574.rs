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

//! The required hardware bits for the 82574 and 82583: Linux
//! e1000_initialize_hw_bits_82571, then the transmit write-back policy and
//! the L1 setting from e1000_init_hw_82571.

use crate::constants::ctrl::{CTRL_82574_HW_BIT29, CTRL_EXT_82574_HW_BIT23, CTRL_EXT_HW_BIT22};
use crate::constants::regs::{
    REG_CTRL, REG_CTRL_EXT, REG_GCR, REG_GCR2, REG_TARC0, REG_TXDCTL0, REG_TXDCTL1,
};
use crate::constants::rxtx::{
    GCR_HW_BIT22, GCR_L1_ACT_WITHOUT_L0S_RX, TXDCTL_COUNT_DESC, TXDCTL_FULL_TX_DESC_WB,
    TXDCTL_WTHRESH,
};
use crate::regs::Regs;

/// TARC0 bits 30:27 cleared, bit 26 set.
const TARC0_CLEAR: u32 = 0xF << 27;
const TARC0_SET: u32 = 1 << 26;
/// GCR2 bit 0: the errata fix for unreliable PCIe completions under ASPM,
/// which otherwise end in transmit timeouts.
const GCR2_COMPLETION_FIX: u32 = 1;

pub fn run(regs: &Regs) {
    // SAFETY: `regs` is the broker-mapped BAR0 window; every offset here is
    // a 4-byte register inside it on the 82574 and 82583.
    unsafe {
        for txdctl in [REG_TXDCTL0, REG_TXDCTL1] {
            regs.modify(txdctl, 0, TXDCTL_COUNT_DESC);
        }
        regs.modify(REG_TARC0, TARC0_CLEAR, TARC0_SET);
        regs.modify(REG_CTRL, CTRL_82574_HW_BIT29, 0);
        regs.modify(REG_CTRL_EXT, CTRL_EXT_82574_HW_BIT23, CTRL_EXT_HW_BIT22);
        regs.modify(REG_GCR, 0, GCR_HW_BIT22);
        regs.modify(REG_GCR2, 0, GCR2_COMPLETION_FIX);
        regs.modify(REG_TXDCTL0, TXDCTL_WTHRESH, TXDCTL_FULL_TX_DESC_WB | TXDCTL_COUNT_DESC);
        regs.modify(REG_GCR, 0, GCR_L1_ACT_WITHOUT_L0S_RX);
    }
}

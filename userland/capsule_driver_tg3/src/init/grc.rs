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

//! The DMA control word, the GRC mode and the timer prescaler, as
//! tg3_reset_hw sets them after the rings are laid out.
//!
//! tg3_test_dma leaves tp->dma_rwctrl at DMA_RWCTRL_DIS_CACHE_ALIGNMENT on
//! a 57765_PLUS part whose host wants no DMA boundary (x86, arm64): the
//! write and read command bits it sets first are dropped by
//! tg3_calc_dma_bndry. So the register keeps what the reset left, but for
//! the read request size on A0, with that one bit set.

use crate::constants::chip::CHIPREV_ID_57765_A0;
use crate::constants::grc::*;
use crate::constants::literals::PRESCALER_66MHZ;
use crate::constants::pcicfg::{
    DMA_RWCTRL_CRDRDR_RDMA_MRRS_MSK, DMA_RWCTRL_DIS_CACHE_ALIGNMENT, TG3PCI_DMA_RW_CTRL,
};
use crate::regs::Regs;

/// tp->grc_mode as tg3_reset_hw leaves it: host send BDs, the transmit
/// pseudo-header checksum left to the host, no NIC send rings.
pub fn grc_mode() -> u32 {
    let clear = GRC_MODE_HOST_SENDBDS
        | GRC_MODE_4X_NIC_SEND_RINGS
        | GRC_MODE_NO_TX_PHDR_CSUM
        | GRC_MODE_NO_RX_PHDR_CSUM;
    (GRC_MODE_BASE & !clear) | GRC_MODE_HOST_SENDBDS | GRC_MODE_NO_TX_PHDR_CSUM
}

/// The DMA control word from what the register holds now.
pub fn dma_rw_ctrl(current: u32, rev: u32) -> u32 {
    let mut v = current & !DMA_RWCTRL_DIS_CACHE_ALIGNMENT;
    if rev == CHIPREV_ID_57765_A0 {
        v &= !DMA_RWCTRL_CRDRDR_RDMA_MRRS_MSK;
    }
    v | DMA_RWCTRL_DIS_CACHE_ALIGNMENT
}

/*
 * SAFETY: `regs` carries the broker MmioMap base for BAR0; the DMA control
 * register is the BAR's copy of configuration offset 0x6C, and GRC_MODE and
 * GRC_MISC_CFG are 32-bit registers inside the window.
 */
pub fn program(regs: &Regs, rev: u32) {
    unsafe {
        let dma = dma_rw_ctrl(regs.r32(TG3PCI_DMA_RW_CTRL), rev);
        regs.w32(TG3PCI_DMA_RW_CTRL, dma);
        regs.w32(GRC_MODE, grc_mode() | GRC_MODE_IRQ_ON_MAC_ATTN | GRC_MODE_HOST_STACKUP);
        let misc = regs.r32(GRC_MISC_CFG) & !0xFF;
        regs.w32(GRC_MISC_CFG, misc | PRESCALER_66MHZ << GRC_MISC_CFG_PRESCALAR_SHIFT);
    }
}

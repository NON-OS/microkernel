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

//! The chip states the per-version waits poll, one read each, and the stop
//! request that comes before some of them.

use super::regs::{
    CMD_STOP_REQ, INTR_MITIGATE_RXTX_EMPTY, MCU_LINK_LIST_RDY, MCU_RXTX_EMPTY, REG_INTR_MITIGATE,
    REG_MCU, TXCFG_EMPTY,
};
use crate::constants::regs::{REG_CMD, REG_TX_CONFIG, REG_TX_POLL, TX_POLL_NPQ};
use crate::regs::Regs;

/// Linux rtl_txcfg_empty_cond.
pub fn tx_fifo_empty(regs: &Regs) -> bool {
    // SAFETY: TxConfig (0x40) lies inside every mapped window.
    unsafe { regs.r32(REG_TX_CONFIG) & TXCFG_EMPTY != 0 }
}

/// Linux rtl_rxtx_empty_cond.
pub fn mcu_rxtx_empty(regs: &Regs) -> bool {
    // SAFETY: MCU (0xD3) lies inside every mapped window.
    unsafe { regs.r8(REG_MCU) & MCU_RXTX_EMPTY == MCU_RXTX_EMPTY }
}

/// Linux rtl_rxtx_empty_cond_2 (8125B and later).
pub fn mitigate_rxtx_empty(regs: &Regs) -> bool {
    // SAFETY: IntrMitigate (0xE2) lies inside every mapped window.
    unsafe { regs.r16(REG_INTR_MITIGATE) & INTR_MITIGATE_RXTX_EMPTY == INTR_MITIGATE_RXTX_EMPTY }
}

/// Linux rtl_link_list_ready_cond.
pub fn link_list_ready(regs: &Regs) -> bool {
    // SAFETY: MCU (0xD3) lies inside every mapped window.
    unsafe { regs.r8(REG_MCU) & MCU_LINK_LIST_RDY != 0 }
}

/// Linux rtl_npq_cond: the normal queue is still being polled.
pub fn npq_busy(regs: &Regs) -> bool {
    // SAFETY: TxPoll (0x38) lies inside the window of every chip that asks.
    unsafe { regs.r8(REG_TX_POLL) & TX_POLL_NPQ != 0 }
}

/// ChipCmd StopReq, ahead of a FIFO wait or a reset.
pub fn request_stop(regs: &Regs) {
    // SAFETY: ChipCmd (0x37) lies inside every mapped window.
    unsafe { regs.w8(REG_CMD, regs.r8(REG_CMD) | CMD_STOP_REQ) };
}

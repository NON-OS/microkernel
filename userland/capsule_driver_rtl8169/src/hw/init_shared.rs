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

use super::conds::link_list_ready;
use super::ocp::mac_ocp_modify;
use super::regs::{MCU_NOW_IS_OOB, REG_MCU};
use super::{enable_rxdvgate, hold_ms, wait_for};
use crate::chip::MacVersion;
use crate::constants::regs::{CMD_RX_ENABLE, CMD_TX_ENABLE, REG_CMD};
use crate::log::Line;
use crate::regs::Regs;

/// Linux r8168g_wait_ll_share_fifo_ready: 42 polls 100 us apart.
pub fn wait_ll_share_fifo_ready(regs: &Regs) {
    if !wait_for(5, true, || link_list_ready(regs)) {
        Line::new("rtl8169: MCU LINK_LIST_RDY not set in 5 ms, going on").send();
    }
}

/*
 * The start Linux rtl_hw_init_8168g and rtl_hw_init_8125 share, at probe
 * before the reset: RX data gated off, both engines off, the MAC taken out
 * of the out-of-band mode a management firmware may have left it in, and
 * MAC OCP 0xE8DE bit 14 cleared to rebuild the shared FIFO link list.
 */
pub fn init_shared(regs: &Regs, ver: MacVersion) {
    if let Err(what) = enable_rxdvgate(regs, ver) {
        Line::new("rtl8169: FIFO not empty at init (").text(what).text("), going on").send();
    }
    // SAFETY (each block): ChipCmd and MCU lie inside every mapped window.
    unsafe { regs.w8(REG_CMD, regs.r8(REG_CMD) & !(CMD_TX_ENABLE | CMD_RX_ENABLE)) };
    hold_ms(1);
    // SAFETY: as above.
    unsafe { regs.w8(REG_MCU, regs.r8(REG_MCU) & !MCU_NOW_IS_OOB) };
    mac_ocp_modify(regs, 0xE8DE, 1 << 14, 0);
    wait_ll_share_fifo_ready(regs);
}

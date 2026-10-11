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

//! The engines in tg3_reset_hw's order: the read DMA FIFO fix, statistics,
//! host coalescing, the receive completion and placement blocks, the MAC's
//! DMA enables, the GPIOs and both DMA engines; then blocks.rs.

use crate::constants::blocks::*;
use crate::constants::grc::GRC_LOCAL_CTRL;
use crate::constants::literals::SNDDATAI_STATS_ALL;
use crate::constants::mac::*;
use crate::setup::Driver;

use super::blocks::{self, settle, ON};
use super::{coalesce, modes};

/// The MAC's statistics and DMA enables (tg3_reset_hw's tp->mac_mode).
pub const MAC_MODE_ENABLES: u32 = MAC_MODE_TXSTAT_ENABLE
    | MAC_MODE_RXSTAT_ENABLE
    | MAC_MODE_TDE_ENABLE
    | MAC_MODE_RDE_ENABLE
    | MAC_MODE_FHDE_ENABLE;

/*
 * SAFETY for every access: `d.regs` carries the broker MmioMap base for
 * BAR0 and each offset is a 32-bit block mode or MAC register inside it.
 */
pub fn enable(d: &mut Driver) {
    let regs = d.regs;
    unsafe {
        let v = regs.r32(TG3_RDMA_RSRVCTRL_REG);
        regs.w32(TG3_RDMA_RSRVCTRL_REG, v | TG3_RDMA_RSRVCTRL_FIFO_OFLW_FIX);
        let v = regs.r32(RCVLPC_STATS_ENABLE) & !RCVLPC_STATSENAB_DACK_FIX;
        regs.w32(RCVLPC_STATS_ENABLE, v);
        regs.w32(RCVLPC_STATSCTRL, RCVLPC_STATSCTRL_ENABLE);
        regs.w32(SNDDATAI_STATSENAB, SNDDATAI_STATS_ALL);
        regs.w32(SNDDATAI_STATSCTRL, SNDDATAI_SCTRL);
    }
    coalesce::program(&regs);
    d.modes.mac_mode |= MAC_MODE_ENABLES;
    let current = unsafe { regs.r32(GRC_LOCAL_CTRL) };
    let wp = d.chip.hw.eeprom_write_protect;
    d.modes.grc_local_ctrl = modes::grc_local_ctrl(d.modes.grc_local_ctrl, wp, current);
    d.modes.tx_mode = TX_MODE_ENABLE | TX_MODE_MBUF_LOCKUP_FIX;
    unsafe {
        regs.w32(RCVCC_MODE, ON);
        regs.w32(RCVLPC_MODE, MODE_ENABLE);
        let clear = MAC_MODE_RXSTAT_CLEAR | MAC_MODE_TXSTAT_CLEAR;
        settle(|| regs.w32f(MAC_MODE, d.modes.mac_mode | clear));
        settle(|| regs.w32f(GRC_LOCAL_CTRL, d.modes.grc_local_ctrl));
        settle(|| regs.w32f(WDMAC_MODE, modes::wdmac_mode()));
        settle(|| regs.w32f(RDMAC_MODE, modes::rdmac_mode(d.chip.rev)));
    }
    blocks::enable(&regs, d.modes.tx_mode);
}

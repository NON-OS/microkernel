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

//! The buffer manager's watermarks for a standard MTU, the manager on, and
//! the standard ring's replenish thresholds (tg3_init_bufmgr_config's
//! 57765_PLUS values, tg3_reset_hw, tg3_setup_rxbd_thresholds).

use crate::constants::bufmgr::*;
use crate::constants::layout::RX_BUFFERS;
use crate::constants::timeouts::BLOCK_MODE_MS;
use crate::regs::Regs;

use super::wait;

/// tg3_setup_rxbd_thresholds: half the BD cache, or an eighth of what is
/// posted, whichever is less. tp->rx_std_max_post is the ring's 512.
pub fn std_thresh(posted: u32) -> u32 {
    let nic = TG3_SRAM_RX_STD_BDCACHE_SIZE_5700 / 2;
    let host = (posted / 8).max(1);
    nic.min(host)
}

/// The jumbo ring is off, but a JUMBO_CAPABLE part has its threshold set
/// from the default jumbo count all the same.
pub fn jumbo_thresh() -> u32 {
    let host = (TG3_DEF_RX_JUMBO_RING_PENDING / 8).max(1);
    (TG3_SRAM_RX_JMB_BDCACHE_SIZE_5700 / 2).min(host)
}

/*
 * SAFETY: `regs` carries the broker MmioMap base for BAR0 and every offset
 * is a 32-bit buffer manager or receive BD register inside it.
 */
pub fn program(regs: &Regs) -> Result<(), &'static str> {
    unsafe {
        regs.w32(BUFMGR_MB_RDMA_LOW_WATER, DEFAULT_MB_RDMA_LOW_WATER_5705);
        regs.w32(BUFMGR_MB_MACRX_LOW_WATER, DEFAULT_MB_MACRX_LOW_WATER_57765);
        regs.w32(BUFMGR_MB_HIGH_WATER, DEFAULT_MB_HIGH_WATER_57765);
        regs.w32(BUFMGR_DMA_LOW_WATER, DEFAULT_DMA_LOW_WATER);
        regs.w32(BUFMGR_DMA_HIGH_WATER, DEFAULT_DMA_HIGH_WATER);
        regs.w32(BUFMGR_MODE, BUFMGR_MODE_ON);
    }
    let on =
        wait::until(BLOCK_MODE_MS, || unsafe { regs.r32(BUFMGR_MODE) & BUFMGR_MODE_ENABLE != 0 });
    if !on {
        return Err("buffer manager did not enable");
    }
    unsafe {
        regs.w32(RCVBDI_STD_THRESH, std_thresh(RX_BUFFERS as u32));
        regs.w32(STD_REPLENISH_LWM, TG3_SRAM_RX_STD_BDCACHE_SIZE_5700);
        regs.w32(RCVBDI_JUMBO_THRESH, jumbo_thresh());
        regs.w32(JMB_REPLENISH_LWM, TG3_SRAM_RX_JMB_BDCACHE_SIZE_5700);
    }
    Ok(())
}

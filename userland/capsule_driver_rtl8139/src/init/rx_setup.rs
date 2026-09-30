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

use crate::constants::dma::RX_BUF_DATA_BYTES;
use crate::constants::regs::{
    CMD_RX_ENABLE, CMD_TX_ENABLE, RCR_ACCEPT_BCAST, RCR_ACCEPT_MULTI, RCR_ACCEPT_PHYS,
    RCR_MXDMA_UNLIMITED, RCR_RBLEN_32K, RCR_WRAP, REG_CAPR, REG_CMD, REG_RBSTART, REG_RCR,
};
use crate::setup::Driver;

/// Receive configuration. Written only once the receiver is enabled (see
/// `run`): on parts where RCR does not take while RE is clear, the accept
/// bits would otherwise fall back to their reset value and nothing arrives.
pub const RCR: u32 = RCR_ACCEPT_PHYS
    | RCR_ACCEPT_MULTI
    | RCR_ACCEPT_BCAST
    | RCR_WRAP
    | RCR_MXDMA_UNLIMITED
    | RCR_RBLEN_32K;

pub fn program(driver: &mut Driver) -> Result<(), &'static str> {
    driver.rx_offset = 0;
    driver.pio.w32(REG_RBSTART, driver.rx_device_addr as u32)?;
    driver.pio.w16(REG_CAPR, capr_for(0))
}

pub fn configure(driver: &Driver) -> Result<(), &'static str> {
    driver.pio.w32(REG_RCR, RCR)
}

/*
 * Receive from the top of the ring again, the way Linux 8139too's rx_err
 * does. Used when the header at the read position is one the part never
 * writes for a good frame: after a FIFO overrun real silicon can leave the
 * ring position lost, and waiting on that header would stop receive for good.
 */
pub fn restart(driver: &mut Driver) -> Result<(), &'static str> {
    driver.pio.w8(REG_CMD, CMD_TX_ENABLE)?;
    driver.pio.w8(REG_CMD, CMD_RX_ENABLE | CMD_TX_ENABLE)?;
    configure(driver)?;
    program(driver)
}

fn capr_for(offset: usize) -> u16 {
    ((offset + RX_BUF_DATA_BYTES - 16) % RX_BUF_DATA_BYTES) as u16
}

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

//! The standard receive ring's control block, the jumbo ring off, and the
//! first producer index, as tg3_reset_hw writes them for a JUMBO_CAPABLE
//! part without the large producer ring; then tg3_rings_reset.

use crate::constants::mailbox::{RX_JMB_PROD_IDX_REG, RX_STD_PROD_IDX_REG};
use crate::constants::mailbox::{TG3_64BIT_REG_HIGH, TG3_64BIT_REG_LOW};
use crate::constants::ring::*;
use crate::constants::sram::NIC_SRAM_RX_BUFFER_DESC;
use crate::setup::Driver;

use super::rcbs;

/// The standard ring's MAXLEN_FLAGS on a 57765_PLUS part: the ring's size
/// above, the buffer size each BD declares, shifted by two, below.
pub fn std_maxlen_flags() -> u32 {
    (RX_STD_RING_SIZE as u32) << BDINFO_FLAGS_MAXLEN_SHIFT | RX_STD_DMA_SZ << 2
}

/*
 * SAFETY: `d.regs` carries the broker MmioMap base for BAR0; the BDINFO
 * blocks at RCVDBDI_STD_BD and RCVDBDI_JUMBO_BD and both producer
 * mailboxes are 32-bit registers inside it.
 */
pub fn program(d: &mut Driver) {
    let regs = d.regs;
    let std = d.rx.std_ring_device_addr;
    let host = RCVDBDI_STD_BD + TG3_BDINFO_HOST_ADDR as usize;
    unsafe {
        regs.w32(host + TG3_64BIT_REG_HIGH, (std >> 32) as u32);
        regs.w32(host + TG3_64BIT_REG_LOW, std as u32);
        regs.w32(RCVDBDI_STD_BD + TG3_BDINFO_NIC_ADDR as usize, NIC_SRAM_RX_BUFFER_DESC);
        let jumbo = RCVDBDI_JUMBO_BD + TG3_BDINFO_MAXLEN_FLAGS as usize;
        regs.w32(jumbo, BDINFO_FLAGS_DISABLED);
        regs.w32(RCVDBDI_STD_BD + TG3_BDINFO_MAXLEN_FLAGS as usize, std_maxlen_flags());
        regs.w32(RX_STD_PROD_IDX_REG, d.rx.std_prod as u32);
        regs.w32(RX_JMB_PROD_IDX_REG, 0);
    }
    rcbs::reset(d);
}

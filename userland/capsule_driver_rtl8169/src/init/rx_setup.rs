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

use crate::chip::MacVersion;
use crate::constants::queue::{BUFFER_SIZE, RX_DESC_COUNT};
use crate::constants::regs::{
    DESC_EOR, DESC_OWN, REG_RMS, REG_RXDESC_ADDR_HI, REG_RXDESC_ADDR_LO, REG_RX_CONFIG,
};
use crate::hw::regs::REG_MAR0;
use crate::hw::rx_config;
use crate::queue::desc::{desc_mut, Descriptor};
use crate::queue::RxRing;
use crate::regs::Regs;

pub fn program(regs: &Regs, rx: &RxRing) {
    for i in 0..RX_DESC_COUNT {
        let eor = if i == RX_DESC_COUNT - 1 { DESC_EOR } else { 0 };
        let addr = rx.buffer_da(i);
        let d = Descriptor {
            opts1: DESC_OWN | eor | BUFFER_SIZE as u32,
            opts2: 0,
            addr_lo: addr as u32,
            addr_hi: (addr >> 32) as u32,
        };
        unsafe {
            desc_mut(rx.desc_va, i, d);
        }
    }
    unsafe {
        regs.w16(REG_RMS, BUFFER_SIZE as u16);
        regs.w32(REG_RXDESC_ADDR_LO, rx.desc_da as u32);
        regs.w32(REG_RXDESC_ADDR_HI, (rx.desc_da >> 32) as u32);
    }
}

/// RxConfig for this version, written once the receiver is enabled (see
/// `run`), after the multicast filter is opened as Linux rtl_set_rx_mode
/// opens it for an interface that takes every multicast group.
pub fn configure(regs: &Regs, ver: MacVersion) {
    // SAFETY: MAR0..MAR0+8 and RxConfig lie inside every mapped window.
    unsafe {
        regs.w32(REG_MAR0 + 4, u32::MAX);
        regs.w32(REG_MAR0, u32::MAX);
        regs.w32(REG_RX_CONFIG, rx_config(ver));
    }
}

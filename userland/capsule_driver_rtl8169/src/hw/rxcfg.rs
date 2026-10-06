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
use crate::constants::regs::{
    RX_CONFIG_ACCEPT_BCAST, RX_CONFIG_ACCEPT_MULTI, RX_CONFIG_ACCEPT_PHYS, RX_CONFIG_DMA,
};

// RxConfig bits as Linux names them (r8169_main.c, enum rtl_registers).
const RX128_INT_EN: u32 = 1 << 15;
const RX_MULTI_EN: u32 = 1 << 14;
const RX_FIFO_THRESH: u32 = 7 << 13;
const RX_EARLY_OFF: u32 = 1 << 11;
const RX_PAUSE_SLOT_ON: u32 = 1 << 11;
const RX_FETCH_DFLT_8125: u32 = 8 << 27;

/*
 * Linux rtl_init_rxcfg, per version. The bits above 7 mean different things
 * on different generations: 7 << 13 is the FIFO threshold on the 8169 and
 * 8168b but RX128_INT_EN | RX_MULTI_EN | a reserved bit on the 8168c and
 * later, and the 8125 keeps its descriptor fetch count in bits 27..30, so
 * one value for every chip is wrong on most of them.
 */
pub fn rx_config_base(ver: MacVersion) -> u32 {
    match ver.0 {
        2..=6 | 10..=17 => RX_FIFO_THRESH | RX_CONFIG_DMA,
        18..=24 | 34..=36 | 38 => RX128_INT_EN | RX_MULTI_EN | RX_CONFIG_DMA,
        40..=52 => RX128_INT_EN | RX_MULTI_EN | RX_CONFIG_DMA | RX_EARLY_OFF,
        61 => RX_FETCH_DFLT_8125 | RX_CONFIG_DMA,
        63.. => RX_FETCH_DFLT_8125 | RX_CONFIG_DMA | RX_PAUSE_SLOT_ON,
        _ => RX128_INT_EN | RX_CONFIG_DMA,
    }
}

/// The base with the accept bits Linux rtl_set_rx_mode sets for a plain
/// interface: broadcast, this station, and multicast.
pub fn rx_config(ver: MacVersion) -> u32 {
    rx_config_base(ver) | RX_CONFIG_ACCEPT_BCAST | RX_CONFIG_ACCEPT_PHYS | RX_CONFIG_ACCEPT_MULTI
}

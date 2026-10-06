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

//! RxConfig and TxConfig per version against Linux rtl_init_rxcfg,
//! rtl_set_rx_mode and rtl_set_tx_config_registers, with Linux's constants
//! spelled out: RX128_INT_EN 1<<15, RX_MULTI_EN 1<<14, RX_FIFO_THRESH 7<<13,
//! RX_EARLY_OFF and RX_PAUSE_SLOT_ON 1<<11, RX_DMA_BURST 7<<8,
//! RX_FETCH_DFLT_8125 8<<27, accept bits 0x0E, TXCFG_AUTO_FIFO 1<<7.

use super::model::window;
use crate::chip::MacVersion;
use crate::constants::regs::{REG_RX_CONFIG, REG_TX_CONFIG};
use crate::hw::{rx_config, tx_config};
use crate::init::{rx_configure, tx_configure};
use crate::regs::Regs;

const ACCEPT: u32 = 0x0E;

#[test]
fn rxconfig_is_the_linux_value_for_each_version() {
    let rows: &[(&[u8], u32)] = &[
        (&[2, 3, 4, 5, 6, 10, 14, 17], (7 << 13) | (7 << 8)),
        (&[18, 19, 20, 21, 22, 23, 24, 34, 35, 36, 38], (1 << 15) | (1 << 14) | (7 << 8)),
        (&[40, 42, 43, 44, 46, 48, 51, 52], (1 << 15) | (1 << 14) | (7 << 8) | (1 << 11)),
        (&[61], (8 << 27) | (7 << 8)),
        (&[63, 64, 65, 66], (8 << 27) | (7 << 8) | (1 << 11)),
        (&[7, 8, 9, 25, 26, 28, 29, 30, 31, 32, 33, 37, 39], (1 << 15) | (7 << 8)),
    ];
    for (versions, base) in rows {
        for &v in *versions {
            assert_eq!(rx_config(MacVersion(v)), base | ACCEPT, "VER_{v}");
        }
    }
}

#[test]
fn txconfig_sets_auto_fifo_only_on_the_8168evl_to_8117() {
    let plain = (3 << 24) | (7 << 8);
    for v in [2u8, 17, 25, 33, 39, 61, 63, 66] {
        assert_eq!(tx_config(MacVersion(v)), plain, "VER_{v}");
    }
    for v in [34u8, 35, 38, 40, 46, 51, 52] {
        assert_eq!(tx_config(MacVersion(v)), plain | (1 << 7), "VER_{v}");
    }
}

#[test]
fn configuring_opens_the_multicast_filter_and_writes_both_registers() {
    let bar = window();
    let regs = Regs::new(bar.base());
    rx_configure(&regs, MacVersion(63));
    tx_configure(&regs, MacVersion(63));
    assert_eq!((bar.wrote32(0x08), bar.wrote32(0x0C)), (u32::MAX, u32::MAX), "MAR0..7");
    assert_eq!(bar.wrote32(REG_RX_CONFIG), 0x4000_0F0E);
    assert_eq!(bar.wrote32(REG_TX_CONFIG), 0x0300_0700);
}

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

//! rtw8821c_coex_cfg_init (rtw8821c.c:810) and rtw8821c_coex_cfg_gnt_debug
//! (rtw8821c.c:937), as rtw88 runs them at power-on.

use crate::regs::Mmio;

pub(super) fn cfg_init<M: Mmio>(mmio: &M) {
    // REG_BCN_CTRL |= BIT_EN_BCN_FUNCTION.
    set8(mmio, 0x0550, 1 << 3);
    // REG_BT_TDMA_TIME sample rate field (bits 5:0) = 5.
    let t = mmio.read8(0x0790);
    mmio.write8(0x0790, (t & !0x3F) | 0x05);
    // REG_BT_STAT_CTRL = BT_CNT_ENABLE.
    mmio.write8(0x0778, 0x01);
    // REG_GPIO_MUXCFG |= BIT_BT_PTA_EN | BIT_PO_BT_PTA_PINS.
    set32(mmio, 0x0040, (1 << 5) | (1 << 9), 0);
    // REG_QUEUE_CTRL: BIT_PTA_WL_TX_EN on, BIT_PTA_EDCCA_EN off.
    let q = mmio.read8(0x04C6);
    mmio.write8(0x04C6, (q | (1 << 4)) & !(1 << 5));
    // REG_BT_COEX_V2 |= BIT_GNT_BT_POLARITY.
    let c = mmio.read16(0x0762);
    mmio.write16(0x0762, c | (1 << 12));
    // REG_BT_COEX_TABLE_H + 3: BIT_BCN_QUEUE = BCN_PRI_EN.
    set8(mmio, 0x06CC + 3, 1 << 3);
}

pub(super) fn gnt_debug<M: Mmio>(mmio: &M) {
    // REG_PAD_CTRL1: BTGP_SPI_EN, BTGP_JTAG_EN and LED1DIS off.
    set32(mmio, 0x0064, 0, (1 << 20) | (1 << 24) | (1 << 15));
    // REG_GPIO_MUXCFG: BIT_FSPI_EN off.
    set32(mmio, 0x0040, 0, 1 << 19);
    // REG_SYS_SDIO_CTRL: BIT_SDIO_INT and BIT_DBG_GNT_WL_BT off.
    set32(mmio, 0x0070, 0, (1 << 18) | (1 << 27));
}

pub(super) fn set8<M: Mmio>(mmio: &M, off: usize, bits: u8) {
    let v = mmio.read8(off);
    mmio.write8(off, v | bits);
}

pub(super) fn set32<M: Mmio>(mmio: &M, off: usize, set: u32, clear: u32) {
    let v = mmio.read32(off);
    mmio.write32(off, (v | set) & !clear);
}

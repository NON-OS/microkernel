// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8822c_coex_cfg_gnt_debug` (rtw8822c.c:3070) and
//! `rtw8822c_coex_cfg_init` (rtw8822c.c:2962), as rtw88 runs them; register
//! facts from reg.h.

use crate::phy::bits::{clr8, set32, set8, write8_mask};
use crate::phy::rf::{self, PATH_B};
use crate::regs::Mmio;

/// The debug pins off: BTGP SPI and JTAG, flash SPI, LED1 disable, and the
/// grant debug output.
pub(super) fn gnt_debug<M: Mmio>(mmio: &M) {
    write8_mask(mmio, 0x0066, 1 << 4, 0); // REG_PAD_CTRL1 + 2: BIT_BTGP_SPI_EN >> 16
    write8_mask(mmio, 0x0067, 1 << 0, 0); // + 3: BIT_BTGP_JTAG_EN >> 24
    write8_mask(mmio, 0x0042, 1 << 3, 0); // REG_GPIO_MUXCFG + 2: BIT_FSPI_EN >> 16
    write8_mask(mmio, 0x0065, 1 << 7, 0); // REG_PAD_CTRL1 + 1: BIT_LED1DIS >> 8
    write8_mask(mmio, 0x0073, 1 << 3, 0); // REG_SYS_SDIO_CTRL + 3: BIT_DBG_GNT_WL_BT >> 24
}

pub(super) fn cfg_init<M: Mmio>(mmio: &M) {
    set8(mmio, 0x0550, 1 << 3); // REG_BCN_CTRL |= BIT_EN_BCN_FUNCTION
    write8_mask(mmio, 0x0790, 0x3F, 0x5); // REG_BT_TDMA_TIME: sample rate 5
    mmio.write8(0x0778, 0x01); // REG_BT_STAT_CTRL: counters on
    set32(mmio, 0x0040, 1 << 5); // REG_GPIO_MUXCFG |= BIT_BT_PTA_EN
    set32(mmio, 0x0040, 1 << 9); // |= BIT_PO_BT_PTA_PINS
    set8(mmio, 0x04C6, 1 << 4); // REG_QUEUE_CTRL |= BIT_PTA_WL_TX_EN
    clr8(mmio, 0x04C6, 1 << 5); // BIT_PTA_EDCCA_EN off
    let c = mmio.read16(0x0762); // REG_BT_COEX_V2 |= BIT_GNT_BT_POLARITY
    mmio.write16(0x0762, c | (1 << 12));
    clr8(mmio, 0x04FC, 0x03); // REG_DUMMY_PAGE4_V1: BIT_BTCCA_CTRL off
    rf::write(mmio, PATH_B, RF_MODOPT, 0xF_FFFF, 0x40000);
}

/// Path B's RF_MODOPT, set "to avoid RF parameter error" (rtw88's words).
const RF_MODOPT: u8 = 0x01;

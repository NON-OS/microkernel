// NONOS Operating System (AGPL-3.0-or-later)

//! rtw88 `rtw_set_channel_mac` (mac.c:12) for a 20 MHz 2.4 GHz channel on
//! an 11ac chip: no sub-channel, the 20 MHz RF mode, the 80 MHz MAC clock
//! and its microsecond counters, and the CCK check off. Register facts from
//! reg.h.

use crate::regs::Mmio;

const REG_DATA_SC: usize = 0x0483;
const REG_WMAC_TRXPTCL_CTL: usize = 0x0668;
/// `BIT_RFMOD`: 0 is 20 MHz.
const RFMOD: u32 = (1 << 7) | (1 << 8);
const REG_AFE_CTRL1: usize = 0x0024;
/// `BIT_MAC_CLK_SEL`; `MAC_CLK_HW_DEF_80M` is 0.
const MAC_CLK_SEL: u32 = (1 << 20) | (1 << 21);
const REG_USTIME_TSF: usize = 0x055C;
const REG_USTIME_EDCA: usize = 0x0638;
const MAC_CLK_SPEED: u8 = 80;
const REG_CCK_CHECK: usize = 0x0454;
const CHECK_CCK_EN: u8 = 1 << 7;

pub fn set_20<M: Mmio>(mmio: &M) {
    mmio.write8(REG_DATA_SC, 0);
    let v = mmio.read32(REG_WMAC_TRXPTCL_CTL) & !RFMOD;
    mmio.write32(REG_WMAC_TRXPTCL_CTL, v);
    let v = mmio.read32(REG_AFE_CTRL1) & !MAC_CLK_SEL;
    mmio.write32(REG_AFE_CTRL1, v);
    mmio.write8(REG_USTIME_TSF, MAC_CLK_SPEED);
    mmio.write8(REG_USTIME_EDCA, MAC_CLK_SPEED);
    let v = mmio.read8(REG_CCK_CHECK) & !CHECK_CCK_EN;
    mmio.write8(REG_CCK_CHECK, v);
}

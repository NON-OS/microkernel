// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8822c_mac_init`'s second part: the rest of EDCA, the MAC clock and
//! the beacon timing (rtw8822c.c:2059-2085).

use crate::mac::op::MacOp;

pub(super) const LEN: usize = 19;

/// MAC_CLK_SPEED: the 80 MHz MAC clock, in the microsecond counters.
const MAC_CLK_SPEED: u8 = 80;

pub(super) const OPS: &[MacOp; LEN] = &[
    MacOp::write32(0x050C, 0x0000_A44F), // REG_EDCA_BK_PARAM
    MacOp::write8(0x0512, 0x1C),         // REG_PIFS: WLAN_PIFS_TIME
    MacOp::clear8(0x0521, 1 << 4),       // REG_TX_PTCL_CTRL + 1: BIT_SIFS_BK_EN >> 8
    // REG_RD_CTRL + 1: (BIT_DIS_TXOP_CFE | BIT_DIS_LSIG_CFE | BIT_DIS_STBC_CFE) >> 8.
    MacOp::set8(0x0525, 0x07),
    MacOp::clear32(0x0024, 0x0030_0000), // REG_AFE_CTRL1: BIT_MAC_CLK_SEL off
    MacOp::write8(0x055C, MAC_CLK_SPEED), // REG_USTIME_TSF
    MacOp::write8(0x0638, MAC_CLK_SPEED), // REG_USTIME_EDCA
    // REG_MISC_CTRL |= BIT_EN_FREE_CNT (BIT(3)) | BIT_DIS_SECOND_CCA (BIT(0) | BIT(1)).
    MacOp::set8(0x0577, 0x0B),
    MacOp::clear8(0x05B4, 0x70), // REG_TIMER0_SRC_SEL: BIT_TSFT_SEL_TIMER0
    MacOp::write16(0x0522, 0x0000), // REG_TXPAUSE
    MacOp::write8(0x051B, 0x09), // REG_SLOT: WLAN_SLOT_TIME
    MacOp::write32(0x0544, 0x001B_0005), // REG_RD_NAV_NXT: RDG_NAV 0x05 | TXOP_NAV 0x1B << 16
    MacOp::write16(0x055E, 0x3030), // REG_RXTSF_OFFSET_CCK: CCK 0x30 | OFDM 0x30 << 8
    MacOp::set8(0x0550, 1 << 3), // REG_BCN_CTRL |= BIT_EN_BCN_FUNCTION
    MacOp::write32(0x0540, 0x0000_6404), // REG_TBTT_PROHIBIT: 0x04 | HOLD 0x064 << 8
    MacOp::write8(0x0558, 0x04), // REG_DRVERLYINT: WLAN_DRV_EARLY_INT
    MacOp::write8(0x0551, 0x10), // REG_BCN_CTRL_CLINT0: WLAN_BCN_CTRL_CLT0
    MacOp::write8(0x0559, 0x02), // REG_BCNDMATIM: WLAN_BCN_DMA_TIME
    MacOp::write8(0x055D, 0xFF), // REG_BCN_MAX_ERR: WLAN_BCN_MAX_ERR
];

// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8822c_mac_init`'s last part: WMAC, the receive low-power filter and
//! interrupt migration (rtw8822c.c:2087-2128), then `rtw_drv_info_cfg`'s 11ac
//! tail (mac.c:1364): the PHY status size, the receive-descriptor fix, and
//! the PHY status appended to every frame.

use super::{clear16, set16};
use crate::mac::op::MacOp;

pub(super) const LEN: usize = 30;

/// WLAN_RXPKT_MAX_SZ (12288) >> 9.
const RXPKT_MAX_SZ_512: u8 = (12288u32 >> 9) as u8;
/// (BIT_RXGCK_{VHT,HT,OFDM,CCK}_FIFOTHR(1)) >> 16: bits 26, 24, 22, 20 down 16.
const RXGCK_FIFOTHR: u16 = 0x0550;
/// BIT_SET_RXPSF_PKTLENTHR(0, 1) (1 << 13) | CTRLEN | VHTCHKEN | HTCHKEN
/// | OFDMCHKEN | CCKCHKEN | OFDMRST (bits 12 down to 7).
const RXPSF_CTRL: u16 = 0x3F80;

pub(super) const OPS: &[MacOp; LEN] = &[
    MacOp::write32(0x0620, 0xFFFF_FFFF), // REG_MAR: WLAN_MULTI_ADDR
    MacOp::write32(0x0624, 0xFFFF_FFFF), // REG_MAR + 4
    MacOp::write8(0x06DE, 0x84),         // REG_BBPSF_CTRL + 2: WLAN_RESP_TXRATE
    MacOp::write8(0x0640, 0x21),         // REG_ACKTO: WLAN_ACK_TO
    MacOp::write8(0x0639, 0x6A),         // REG_ACKTO_CCK: WLAN_ACK_TO_CCK
    MacOp::write16(0x0642, 0x0040),      // REG_EIFS: WLAN_EIFS_DUR_TUNE
    MacOp::write8(0x0652, 0xC8),         // REG_NAV_CTRL + 2: WLAN_NAV_MAX
    MacOp::write8(0x066E, 0x05),         // REG_WMAC_TRXPTCL_CTL_H + 2: WLAN_BAR_ACK_TYPE
    MacOp::write32(0x06A0, 0xFFFF_FFFF), // REG_RXFLTMAP0: WLAN_RX_FILTER0
    MacOp::write16(0x06A4, 0xFFFF),      // REG_RXFLTMAP2: WLAN_RX_FILTER2
    MacOp::write32(0x0608, 0xE400_220E), // REG_RCR: WLAN_RCR_CFG
    MacOp::write8(0x060C, RXPKT_MAX_SZ_512), // REG_RX_PKT_LIMIT
    MacOp::write8(0x0606, 0x30),         // REG_TCR + 2: WLAN_TX_FUNC_CFG2
    MacOp::write8(0x0605, 0x30),         // REG_TCR + 1: WLAN_TX_FUNC_CFG1
    MacOp::set32(0x1664, 1 << 9),        // REG_GENERAL_OPTION: DUMMY_FCS_READY_MASK_EN
    MacOp::write32(0x07D8, 0xB081_0041), // REG_WMAC_OPTION_FUNCTION + 8: OPT_FUNC2
    MacOp::write8(0x07D4, 0x98),         // REG_WMAC_OPTION_FUNCTION_1: NORM_FUNC1
    // REG_RXPSF_CTRL + 2: (value & 0xF00F) | RXGCK_FIFOTHR.
    clear16(0x1612, 0x0FF0),
    set16(0x1612, RXGCK_FIFOTHR),
    MacOp::write16(0x1610, RXPSF_CTRL),  // REG_RXPSF_CTRL
    MacOp::write32(0x1614, 0xFFFF_FFFF), // REG_RXPSF_TYPE_CTRL
    // REG_RXPSF_CTRL: MHCHKEN, CCKRST and CONT_ERRCHKEN off, ERRTHR = 7.
    clear16(0x1610, 0x0077),
    set16(0x1610, 0x0007),
    MacOp::set8(0x0718, 1 << 6), // REG_SND_PTCL_CTRL: DIS_CHK_VHTSIGB_CRC
    MacOp::write32(0x0304, 0x3333_0000), // REG_INT_MIG: WLAN_MAC_INT_MIG_CFG
    MacOp::write8(0x060F, 4),    // REG_RX_DRVINFO_SZ: PHY_STATUS_SIZE
    // REG_TRXFF_BNDY + 1: (value & 0xF0) | 0xF, "for rxdesc len = 0 issue".
    MacOp::clear8(0x0115, 0x0F),
    MacOp::set8(0x0115, 0x0F),
    MacOp::set32(0x0608, 1 << 28), // REG_RCR |= BIT_APP_PHYSTS
    MacOp::clear32(0x07D4, (1 << 8) | (1 << 9)), // REG_WMAC_OPTION_FUNCTION + 4
];

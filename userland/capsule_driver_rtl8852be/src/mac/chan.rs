// NONOS Operating System (AGPL-3.0-or-later)

//! `__rtw8852bx_set_channel_mac` (Linux v6.12 rtw89 `rtw8852b_common.c`)
//! at 20 MHz, and the PPDU status report switch rtw89's channel help turns
//! off and on around a switch (`rtw89_mac_cfg_ppdu_status_ax`, `mac.c`).

use super::{cmac_on, MacFail};
use crate::regs::{Mmio, Rw};

const R_AX_WMAC_RFMOD: u32 = 0xc010;
const R_AX_TX_SUB_CARRIER_VALUE: u32 = 0xc088;
const R_AX_TXRATE_CHK: u32 = 0xc628;
const BAND_MODE: u8 = 0x10;
/// B_AX_CHECK_CCK_EN | B_AX_RTS_LIMIT_IN_OFDM6.
const OFDM_ONLY: u8 = 0x1 | 0x2;
pub const R_AX_PPDU_STAT: u32 = 0xce40;
/// RPT_EN, APP_MAC_INFO, APP_RX_CNT, APP_PLCP_HDR, RPT_CRC32.
pub const PPDU_STAT_ON: u32 = 0x1 | 0x2 | 0x4 | 0x8 | 0x20;
const R_AX_HW_RPT_FWD: u32 = 0x9c18;
const FWD_PPDU_STAT: u32 = 0x3;
const PRPT_DEST_HOST: u32 = 1;

pub fn set_channel_mac(r: &impl Mmio, five: bool) {
    r.clr8(R_AX_WMAC_RFMOD, 0x3);
    r.write32(R_AX_TX_SUB_CARRIER_VALUE, 0);
    if five {
        r.clr8(R_AX_TXRATE_CHK, BAND_MODE);
        r.set8(R_AX_TXRATE_CHK, OFDM_ONLY);
    } else {
        r.set8(R_AX_TXRATE_CHK, BAND_MODE);
        r.clr8(R_AX_TXRATE_CHK, OFDM_ONLY);
    }
}

pub fn ppdu_status(r: &impl Mmio, on: bool) -> Result<(), MacFail> {
    cmac_on(r)?;
    if !on {
        r.clr32(R_AX_PPDU_STAT, 0x1);
        return Ok(());
    }
    r.write32(R_AX_PPDU_STAT, PPDU_STAT_ON);
    r.mask32(R_AX_HW_RPT_FWD, FWD_PPDU_STAT, PRPT_DEST_HOST);
    Ok(())
}

// NONOS Operating System (AGPL-3.0-or-later)

//! `__rtw8852bx_bb_cfg_txrx_path` (Linux v6.12 rtw89 `rtw8852b_common.c`)
//! with both RX paths (rtw89's default, `RF_AB`, two spatial streams):
//! `__rtw8852bx_bb_ctrl_rx_path` (which also sets the gain offsets and,
//! on 2 GHz, lets BT share path B: `__rtw8852bx_ctrl_btg_bt_rx`),
//! `rtw8852bx_bb_ctrl_rf_mode_rx_path`, the NSS limits and the MAC select.

use super::bb::Bb;
use super::ch_gain::gain_offset;
use super::state::Phy;
use crate::chan::Chan;
use crate::regs::Mmio;

/// `__rtw8852bx_ctrl_btg_bt_rx`: (register, mask, value on, value off).
pub const BTG_BT_RX: [(u32, u32, u32, u32); 11] = [
    (0x4738, 0x8_0000, 1, 0),          // R_PATH0_BT_SHARE_V1
    (0x4738, 0x40_0000, 0, 0),         // R_PATH0_BTG_PATH_V1
    (0x476c, 0xff00_0000, 0x20, 0x1a), // R_PATH1_G_LNA6_OP1DB_V1
    (0x4778, 0xff, 0x30, 0x2a),        // R_PATH1_G_TIA0_LNA6_OP1DB_V1
    (0x4aa4, 0x8_0000, 1, 0),          // R_PATH1_BT_SHARE_V1
    (0x4aa4, 0x40_0000, 1, 0),         // R_PATH1_BTG_PATH_V1
    (0x980, 0x1e_0000, 0, 0xc),        // R_PMAC_GNT B_PMAC_GNT_P1
    (0x49c4, 0x4000, 1, 0),            // R_CHBW_MOD_V1 B_BT_SHARE
    (0x49c0, 0x3c0_0000, 2, 0),        // R_FC0_BW_V1 B_ANT_RX_BT_SEG0
    (0x4420, 0x8000_0000, 1, 1),       // R_BT_DYN_DC_EST_EN_V1
    (0xc6c, 0x20_0000, 1, 0),          // R_GNT_BT_WGT_EN
];

pub fn btg_bt_rx(bb: &Bb<'_, impl Mmio>, on: bool) {
    for &(a, m, v_on, v_off) in BTG_BT_RX.iter() {
        bb.mask(a, m, if on { v_on } else { v_off });
    }
}

/// `rx_nss` is what the firmware reported (`hal.rx_nss`), 1 or 2.
pub fn cfg_txrx_path(r: &impl Mmio, p: &Phy, c: Chan, rx_nss: u8) {
    let bb = Bb(r);
    bb.mask(0x49c4, 0xf, 3); // R_CHBW_MOD_V1 B_ANT_RX_SEG0
    bb.mask(0x49c0, 0x3_c000, 3); // B_ANT_RX_1RCCA_SEG0
    bb.mask(0x49c0, 0x3c_0000, 3); // B_ANT_RX_1RCCA_SEG1
    bb.mask(0xd18, 0x300, 1); // R_RXHT_MCS_LIMIT
    bb.mask(0xd18, 0x60_0000, 1); // R_RXVHT_MCS_LIMIT
    bb.mask(0xd80, 0x3fc0, 4); // R_RXHE B_RXHE_USER_MAX
    bb.mask(0xd80, 0x1_c000, 1); // B_RXHE_MAX_NSS
    bb.mask(0xd80, 0x380_0000, 1); // B_RXHETB_MAX_NSS
    gain_offset(&bb, p, c);
    btg_bt_rx(&bb, !c.five);
    bb.mask(0x78dc, 0xc000_0000, 1); // R_P1_TXPW_RSTB MANON|TSSI
    bb.mask(0x78dc, 0xc000_0000, 3);
    for (mode, ftm) in [(0x12ac, 0x12b0), (0x32ac, 0x32b0)] {
        bb.mask(mode, 0xffff_fff0, 0x1233312); // RFMODE_ORI_TXRX_FTM_TX
        bb.mask(ftm, 0xfff, 0x333); // RFMODE_FTM_RX
    }
    nss(&bb, (rx_nss > 1) as u32);
    bb.mask(0x9a4, 0x1c, 0); // R_MAC_SEL B_MAC_SEL_MOD
}

/// The HT, VHT, HE and HE TB stream limits: 1 for two streams, 0 for one.
fn nss(bb: &Bb<'_, impl Mmio>, v: u32) {
    bb.mask(0xd18, 0x300, v); // R_RXHT_MCS_LIMIT
    bb.mask(0xd18, 0x60_0000, v); // R_RXVHT_MCS_LIMIT
    bb.mask(0xd80, 0x1_c000, v); // R_RXHE B_RXHE_MAX_NSS
    bb.mask(0xd80, 0x380_0000, v); // B_RXHETB_MAX_NSS
}

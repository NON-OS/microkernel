// NONOS Operating System (AGPL-3.0-or-later)

//! `__rtw8852bx_set_channel_bb` (Linux v6.12 rtw89 `rtw8852b_common.c`) for
//! a 20 MHz channel: the CCK SCO thresholds on 2 GHz, `rtw8852bx_ctrl_ch`,
//! `rtw8852bx_ctrl_bw` with `rtw8852b_bw_setting` at 20 MHz, CCK on or
//! off, the BT share undone on 5 GHz, the channel index, the 5 MHz mask
//! off (only 40/80 MHz set it), and the BB reset.

use super::bb::Bb;
use super::ch_ctrl::ctrl_ch;
use super::reset::reset_all;
use super::state::Phy;
use crate::chan::Chan;
use crate::regs::Mmio;

const BARKER: [u32; 14] = [
    0x1cfea, 0x1d0e1, 0x1d1d7, 0x1d2cd, 0x1d3c3, 0x1d4b9, 0x1d5b0, 0x1d6a6, 0x1d79c, 0x1d892,
    0x1d988, 0x1da7f, 0x1db75, 0x1ddc4,
];
const CCK: [u32; 14] = [
    0x27de3, 0x27f35, 0x28088, 0x281da, 0x2832d, 0x2847f, 0x285d2, 0x28724, 0x28877, 0x289c9,
    0x28b1c, 0x28c6e, 0x28dc1, 0x290ed,
];
const R_CHBW_MOD_V1: u32 = 0x49c4;
const R_FC0_BW_V1: u32 = 0x49c0;
const R_P0_RFMODE_ORI_RX: u32 = 0x12ac;
const R_P1_RFMODE_ORI_RX: u32 = 0x32ac;
const RFMODE_ORI_RX_ALL: u32 = 0xfff000;

pub fn set_channel_bb(r: &impl Mmio, p: &Phy, c: Chan) {
    let bb = Bb(r);
    if !c.five {
        bb.mask(0x23b0, 0x7ffff, BARKER[c.ch as usize - 1]); // R_RXSCOBC
        bb.mask(0x23b4, 0x7ffff, CCK[c.ch as usize - 1]); // R_RXSCOCCK
    }
    ctrl_ch(&bb, p, c);
    ctrl_bw20(&bb);
    bb.mask(0x700, 0x20, (!c.five) as u32); // R_UPD_CLK_ADC B_ENABLE_CCK
    bb.mask(0x2344, 0x8000_0000, c.five as u32); // R_RXCCA B_RXCCA_DIS
    if c.five {
        // R_PATH0/1_BT_SHARE_V1 and R_PATH0/1_BTG_PATH_V1.
        for (a, m) in [(0x4738, 0x8_0000), (0x4738, 0x40_0000), (0x4aa4, 0x8_0000)] {
            bb.mask(a, m, 0);
        }
        bb.mask(0x4aa4, 0x40_0000, 0);
        bb.mask(R_CHBW_MOD_V1, 0x4000, 0); // B_BT_SHARE
        bb.mask(R_FC0_BW_V1, 0x3c0_0000, 0); // B_ANT_RX_BT_SEG0
        bb.mask(0x4420, 0x8000_0000, 0); // R_BT_DYN_DC_EST_EN_V1
        bb.mask(0xc6c, 0x20_0000, 0); // R_GNT_BT_WGT_EN
    }
    bb.mask(0x734, 0xff_0000, c.chan_idx() as u32); // R_MAC_PIN_SEL B_CH_IDX_SEG0
    bb.mask(0x46f8, 0x1000, 0); // R_PATH0_5MDET_V1 EN
    bb.mask(0x47b8, 0x1000, 0); // R_PATH1_5MDET_V1 EN
    bb.mask(0x4440, 0x8000_0000, 0); // R_ASSIGN_SBD_OPT_V1
    reset_all(r);
}

/// `rtw8852bx_ctrl_bw` at RTW89_CHANNEL_WIDTH_20 on the 8852B.
pub fn ctrl_bw20(bb: &Bb<'_, impl Mmio>) {
    let rx_path_0 = bb.get(R_CHBW_MOD_V1, 0xf);
    bb.mask(R_FC0_BW_V1, 0xc000_0000, 0); // B_FC0_BW_SET
    bb.mask(R_CHBW_MOD_V1, 0x3000, 0); // B_CHBW_MOD_SBW
    bb.mask(R_CHBW_MOD_V1, 0xf00, 0); // B_CHBW_MOD_PRICH
    bb.mask(R_P0_RFMODE_ORI_RX, RFMODE_ORI_RX_ALL, 0x333);
    bb.mask(R_P1_RFMODE_ORI_RX, RFMODE_ORI_RX_ALL, 0x333);
    for (adc, wbadc) in [(0xc0ec, 0xc0e4), (0xc1ec, 0xc1e4)] {
        bb.mask(adc, 0x6000, 0);
        bb.mask(wbadc, 0x30, 2);
    }
    match rx_path_0 {
        1 => bb.mask(R_P1_RFMODE_ORI_RX, RFMODE_ORI_RX_ALL, 0x111),
        2 => bb.mask(R_P0_RFMODE_ORI_RX, RFMODE_ORI_RX_ALL, 0x111),
        _ => {}
    }
}

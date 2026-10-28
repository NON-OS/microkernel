// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8852bx_ctrl_ch` and `rtw8852bx_set_rxsc_rpl_comp` (Linux v6.12
//! rtw89 `rtw8852b_common.c`): band select per path, the SCO compensation,
//! the CCK TX filter (channel 14 has its own), the RX gains, and the RPL
//! offsets averaged over the two paths.

use super::bb::Bb;
use super::ch_gain::{gain_error, gain_offset};
use super::rpl::rpl_comp;
use super::state::Phy;
use crate::chan::Chan;
use crate::regs::Mmio;

/// `rtw8852bx_sco_mapping`.
pub const fn sco(ch: u8) -> u32 {
    match ch {
        1 => 109,
        2..=6 => 108,
        7..=10 => 107,
        11..=14 => 106,
        36 | 38 => 51,
        40..=58 => 50,
        60..=64 => 49,
        100 | 102 => 48,
        104..=126 => 47,
        128..=151 => 46,
        153..=177 => 45,
        _ => 0,
    }
}

/// R_TXFIR0..R_TXFIRE (each mask 0xffffff): channel 14, then the rest.
pub const TXFIR: [[u32; 8]; 2] = [
    [0x3b13ff, 0x1c42de, 0xfdb0ad, 0xf60f6e, 0xfd8f92, 0x2d011, 0x1c02c, 0xfff00a],
    [0x3d23ff, 0x29b354, 0xfc1c8, 0xfdb053, 0xf86f9a, 0xfaef92, 0xfe5fcc, 0xffdff5],
];

pub fn ctrl_ch(bb: &Bb<'_, impl Mmio>, p: &Phy, c: Chan) {
    let g = (!c.five) as u32;
    bb.mask(0x4738, 0x2_0000, g); // R_PATH0_BAND_SEL_V1
    bb.mask(0x4aa4, 0x2_0000, g); // R_PATH1_BAND_SEL_V1
    bb.mask(0x49c0, 0x7f, sco(c.ch)); // R_FC0_BW_V1 B_FC0_BW_INV
    let fir = &TXFIR[(c.ch != 14) as usize];
    for (i, &v) in fir.iter().enumerate() {
        bb.mask(0x2300 + 4 * i as u32, 0xff_ffff, v);
    }
    gain_error(bb, p, c, 0);
    gain_error(bb, p, c, 1);
    gain_offset(bb, p, c);
    rpl_comp(bb, p, c);
}

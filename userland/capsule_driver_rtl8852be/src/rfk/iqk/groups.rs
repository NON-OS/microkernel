// NONOS Operating System (AGPL-3.0-or-later)

//! Wideband TX and RX IQK over rtw89's four gain groups (Linux v6.12 rtw89
//! `rtw8852b_rfk.c`; TX is in `txk`): `_iqk_rxclk_setting` at 20 MHz,
//! `_iqk_rxk_setting` and `_rxk_group_sel`. rtw89's 8852B never takes the
//! narrowband paths (`is_nbiqk` stays false), and neither does this.

use super::shot::{one_shot, Shot};
use super::txset::clk;
use crate::phy::bb::Bb;
use crate::phy::rf::RFREG_MASK;
use crate::regs::Mmio;

pub const R_CFIR_LUT: u32 = 0x8154;
pub const R_IQKINF: u32 = 0x9fe0;
pub const R_IQK_RES: u32 = 0x8124;
/// (RX gain, attenuation 2, attenuation 1) per group: 2G, then 5G.
const RX_GROUPS: [[(u32, u32, u32); 4]; 2] = [
    [(0x212, 0x00, 0x3), (0x21c, 0x00, 0x3), (0x350, 0x28, 0x2), (0x360, 0x5f, 0x1)],
    [(0x190, 0x0f, 0x3), (0x198, 0x0f, 0x1), (0x350, 0x3f, 0x0), (0x352, 0x7f, 0x0)],
];

/// `_iqk_rxclk_setting` at 20 MHz, then `_iqk_rxk_setting`.
pub fn rx_setup(bb: &Bb<'_, impl Mmio>, path: u8, five: bool) {
    clk(bb, [0x0f, 0x03], [0xa001, 0xa041]);
    for rxck in [0x12a0, 0x32a0] {
        bb.mask(rxck, 0x7_0000, 1); // RXCK VAL
        bb.mask(rxck, 0x8_0000, 1); // RXCK ON
    }
    bb.mask(0x700, 0x100_0000, 1); // R_UPD_CLK_ADC ON
    bb.mask(0x700, 0x600_0000, 0); // VAL
    bb.write_rf(path, 0x0, 0xf_0000, 0xc); // RR_MOD
    bb.write_rf(path, 0x20, if five { 0x80 } else { 0x100 }, 1); // RR_RXK SEL5G/2G
    let v = bb.read_rf(path, 0x18, RFREG_MASK); // RR_CFGCH
    bb.write_rf(path, 0x1f, RFREG_MASK, v); // RR_RSV4
}

/// `_rxk_group_sel`; true when any group failed.
pub fn rxk(bb: &Bb<'_, impl Mmio>, path: u8, five: bool) -> bool {
    let (lut, p) = (R_CFIR_LUT + ((path as u32) << 8), path as u32);
    let mut kfail = false;
    for (gp, &(gain, att2, att1)) in RX_GROUPS[five as usize].iter().enumerate() {
        bb.write_rf(path, 0x0, 0x3ff0, gain); // RR_MOD RGM
        if five {
            bb.write_rf(path, 0x8c, 0x7f, att2); // RR_RXA2 HATT
            bb.write_rf(path, 0x8c, 0x180, att1); // CC2
        } else {
            bb.write_rf(path, 0x83, 0x1_fc00, att2); // RR_RXBB C2G
            bb.write_rf(path, 0x83, 0x300, att1); // C1G
        }
        bb.mask(lut, 0x100, 1);
        bb.mask(lut, 0x10, 0);
        bb.mask(lut, 0x7, gp as u32); // GP_V1
        let fail = one_shot(bb, path, Shot::Rxk(0));
        bb.mask(R_IQKINF, 1 << (16 + gp as u32 + p * 4), fail as u32);
        kfail |= fail;
    }
    bb.write_rf(path, 0x20, 0x80, 0); // RR_RXK SEL5G off
    bb.mask(R_IQK_RES + (p << 8), 0xf, if kfail { 0 } else { 5 }); // RXCFIR
    kfail
}

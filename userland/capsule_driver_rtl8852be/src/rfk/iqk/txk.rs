// NONOS Operating System (AGPL-3.0-or-later)

//! Wideband TX IQK over rtw89's four gain groups, `_txk_group_sel` (Linux
//! v6.12 rtw89 `rtw8852b_rfk.c`).

use super::groups::{R_CFIR_LUT, R_IQKINF, R_IQK_RES};
use super::lok::R_KIP_IQP;
use super::shot::{one_shot, Shot, R_NCTL_N1};
use crate::phy::bb::Bb;
use crate::regs::Mmio;

/// (power range, track range, BB gain, ITQT) per group: 2G, then 5G.
const TX_GROUPS: [[(u32, u32, u32, u32); 4]; 2] = [
    [(0, 0x4, 0x08, 0x09), (0, 0x4, 0x0e, 0x12), (0, 0x6, 0x06, 0x1b), (0, 0x6, 0x0e, 0x24)],
    [(0, 0x3, 0x08, 0x12), (0, 0x3, 0x0e, 0x12), (0, 0x6, 0x06, 0x12), (0, 0x6, 0x0e, 0x1b)],
];

/// `_txk_group_sel`; true when any group failed.
pub fn txk(bb: &Bb<'_, impl Mmio>, path: u8, five: bool) -> bool {
    let (lut, p) = (R_CFIR_LUT + ((path as u32) << 8), path as u32);
    let mut kfail = false;
    for (gp, &(pr, tr, g, itqt)) in TX_GROUPS[five as usize].iter().enumerate() {
        bb.write_rf(path, 0x11, 0x3, pr); // RR_TXIG GR0
        bb.write_rf(path, 0x11, 0x70, tr); // GR1
        bb.write_rf(path, 0x11, 0x1_f000, g); // TG
        bb.mask(R_KIP_IQP + (p << 8), u32::MAX, itqt);
        bb.mask(lut, 0x100, 1); // SEL
        bb.mask(lut, 0x10, 1); // SET
        bb.mask(lut, 0x4, 0); // G2
        bb.mask(lut, 0x3, gp as u32); // GP
        bb.mask(R_NCTL_N1, 0xff, 0); // CIP
        let fail = one_shot(bb, path, Shot::Txk(0));
        bb.mask(R_IQKINF, 1 << (8 + gp as u32 + p * 4), fail as u32);
        kfail |= fail;
    }
    bb.mask(R_IQK_RES + (p << 8), 0xf00, if kfail { 0 } else { 5 }); // TXCFIR
    kfail
}

// NONOS Operating System (AGPL-3.0-or-later)

//! LO leakage calibration (Linux v6.12 rtw89 `rtw8852b_rfk.c`): `_iqk_lok`
//! and `_lok_finetune_check`; the TX setup around it is in `txset`.

use super::shot::{dif4_txt, one_shot, Shot};
use crate::phy::bb::Bb;
use crate::phy::rf::RFREG_MASK;
use crate::regs::Mmio;

const RR_TXIG: u32 = 0x11;
const TXIG_GR0: u32 = 0x3;
const TXIG_GR1: u32 = 0x70;
const TXIG_TG: u32 = 0x1_f000;
pub const R_KIP_IQP: u32 = 0x81cc;

/// `_iqk_lok`; true when the result is out of range.
pub fn lok(bb: &Bb<'_, impl Mmio>, path: u8, five: bool) -> [bool; 3] {
    let iqp = R_KIP_IQP + ((path as u32) << 8);
    let tg = |v| bb.write_rf(path, RR_TXIG, TXIG_TG, v);
    dif4_txt(bb, 0x021);
    bb.write_rf(path, RR_TXIG, TXIG_GR0, 0);
    bb.write_rf(path, RR_TXIG, TXIG_GR1, if five { 0x4 } else { 0x6 });
    tg(0);
    bb.mask(iqp, u32::MAX, 0x9);
    let coarse = one_shot(bb, path, Shot::FlokCoarse);
    tg(0x12);
    bb.mask(iqp, u32::MAX, 0x24);
    one_shot(bb, path, Shot::FlokVbuffer);
    tg(0);
    bb.mask(iqp, u32::MAX, 0x9);
    dif4_txt(bb, 0x021);
    let fine = one_shot(bb, path, Shot::FlokFine);
    tg(0x12);
    bb.mask(iqp, u32::MAX, 0x24);
    one_shot(bb, path, Shot::FlokVbuffer);
    [coarse, fine, finetune_bad(bb, path)]
}

/// `_lok_finetune_check`.
pub fn finetune_bad(bb: &Bb<'_, impl Mmio>, path: u8) -> bool {
    let t = bb.read_rf(path, 0x58, RFREG_MASK); // RR_TXMO
    let (ci, cq) = ((t & 0xf_8000) >> 15, (t & 0x7c00) >> 10);
    let v = bb.read_rf(path, 0xa, RFREG_MASK); // RR_LOKVB
    let (vi, vq) = ((v & 0xf_c000) >> 14, (v & 0x3f0) >> 4);
    let bad = |x: u32, hi| !(0x2..=hi).contains(&x);
    bad(ci, 0x1d) || bad(cq, 0x1d) || bad(vi, 0x3d) || bad(vq, 0x3d)
}

// NONOS Operating System (AGPL-3.0-or-later)

//! IQK's setup and teardown (Linux v6.12 rtw89 `rtw8852b_rfk.c`):
//! `_iqk_get_ch_info`, `_iqk_preset` and `_iqk_info_iqk` (what is saved
//! around them, and `_iqk_restore`, are in `save`).

use super::groups::{R_IQKINF, R_IQK_RES};
use super::shot::R_NCTL_RPT;
use super::Iqk;
use crate::chan::Chan;
use crate::phy::bb::Bb;
use crate::regs::Mmio;

const R_IQKINF2: u32 = 0x9fe8;
pub const R_TXIQC: u32 = 0x8138;
pub const R_RXIQC: u32 = 0x813c;

pub fn ch_info(bb: &Bb<'_, impl Mmio>, s: &mut Iqk, path: u8, c: Chan) {
    let p = path as usize;
    let idx = match (0..2).find(|&i| s.mcc_ch[i][p] == 0) {
        Some(i) => i,
        None if s.table_idx[p] + 1 > 1 => 0,
        None => (s.table_idx[p] + 1) as usize,
    };
    s.mcc_ch[idx][p] = c.ch;
    s.table_idx[p] = idx as u8;
    let sh = (path as u32) * 16;
    bb.mask(R_IQKINF, 0xff00_0000, 0x2a); // B_IQKINF_VER, RTW8852B_IQK_VER
    bb.mask(0x9fe4, 0xf << sh, c.five as u32); // R_IQKCH band
    bb.mask(0x9fe4, 0xf0 << sh, 0); // bandwidth 20 MHz
    bb.mask(0x9fe4, 0xff00 << sh, c.ch as u32); // channel
}

pub fn preset(bb: &Bb<'_, impl Mmio>, s: &Iqk, path: u8) {
    let (idx, p) = (s.table_idx[path as usize] as u32, (path as u32) << 8);
    bb.mask(0x8104 + p, 0x1, idx); // R_COEF_SEL IQC
    bb.mask(0x8154 + p, 0x8, idx); // R_CFIR_LUT G3
    bb.write_rf(path, 0x5, 0x1, 0); // RR_RSV1 RST
    bb.write_rf(path, 0x10005, 0x1, 0); // RR_BBDC SEL
    bb.mask(R_NCTL_RPT, u32::MAX, 0x80);
    bb.mask(0x8088, u32::MAX, 0x81ff_010a); // R_KIP_SYSCFG
}

pub fn info(bb: &Bb<'_, impl Mmio>, s: &mut Iqk, path: u8) {
    let sh = (path as u32) * 4;
    for (bit, &f) in [0x1u32, 0x2, 0x4, 0x8].iter().zip(s.fail[path as usize].iter()) {
        bb.mask(R_IQKINF, bit << sh, f as u32);
    }
    let p = (path as u32) << 8;
    for a in [R_IQK_RES + p, R_TXIQC + p, R_RXIQC + p] {
        let _ = bb.read32(a); // rtw89 keeps these for its debugfs
    }
    bb.mask(R_IQKINF2, 0xff00, s.times as u32);
    if bb.get(R_IQKINF, 0xf << sh) != 0 {
        s.fail_cnt = s.fail_cnt.wrapping_add(1);
    }
    bb.mask(R_IQKINF2, 0xff_0000 << sh, s.fail_cnt as u32);
}

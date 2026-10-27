// NONOS Operating System (AGPL-3.0-or-later)

//! What IQK saves and puts back (Linux v6.12 rtw89 `rtw8852b_rfk.c`):
//! `_rfk_backup_bb_reg`/`_rf_reg` with their restores, and the non-DBCC
//! BB lists `_iqk_macbb_setting` writes and `_iqk_afebb_restore` undoes,
//! and `_iqk_restore`, which loads the results.

use super::setup::{R_RXIQC, R_TXIQC};
use super::shot::{check_cal, R_NCTL_CFG, R_NCTL_N1, R_NCTL_RPT};
use crate::phy::bb::Bb;
use crate::phy::rf::RFREG_MASK;
use crate::regs::Mmio;
use crate::rfk::reg5::{run, List};

const BB_REGS: [u32; 3] = [0x2344, 0x5800, 0x7800];
const RF_REGS: [u32; 11] = [0xde, 0xdf, 0x8b, 0x90, 0x97, 0x85, 0x1e, 0x0, 0x2, 0x5, 0x10005];

pub fn backup(bb: &Bb<'_, impl Mmio>, path: u8) -> ([u32; 3], [u32; 11]) {
    (BB_REGS.map(|a| bb.read32(a)), RF_REGS.map(|a| bb.read_rf(path, a, RFREG_MASK)))
}

pub fn restore_backup(bb: &Bb<'_, impl Mmio>, path: u8, saved: &([u32; 3], [u32; 11])) {
    for (a, v) in BB_REGS.iter().zip(saved.0) {
        bb.mask(*a, u32::MAX, v);
    }
    for (a, v) in RF_REGS.iter().zip(saved.1) {
        bb.write_rf(path, *a, RFREG_MASK, v);
    }
}

pub fn macbb(bb: &Bb<'_, impl Mmio>) {
    run(bb, List::IqkSetNonDbcc);
}

pub fn afebb_restore(bb: &Bb<'_, impl Mmio>) {
    run(bb, List::IqkRestoreNonDbcc);
}

/// `_iqk_restore` with the narrowband CFIR words the groups left.
pub fn restore(bb: &Bb<'_, impl Mmio>, path: u8, tx_cfir: u32, rx_cfir: u32) {
    let p = (path as u32) << 8;
    bb.mask(R_TXIQC + p, u32::MAX, tx_cfir);
    bb.mask(R_RXIQC + p, u32::MAX, rx_cfir);
    bb.mask(R_NCTL_CFG, u32::MAX, 0xe19 + ((path as u32) << 4));
    let _ = check_cal(bb);
    bb.mask(R_NCTL_N1, 0xff, 0);
    bb.mask(R_NCTL_RPT, u32::MAX, 0);
    bb.mask(0x8088, u32::MAX, 0x8000_0000); // R_KIP_SYSCFG
    bb.mask(0x8120, 0x1000_0000, 0); // R_CFIR_SYS B_IQK_RES_K
    bb.mask(0x8220, 0x1000_0000, 0); // R_IQRSN K1
    bb.mask(0x8220, 0x1_0000, 0); // K2
    bb.write_rf(path, 0xef, 0x4, 0); // RR_LUTWE LOK, twice as rtw89
    bb.write_rf(path, 0xef, 0x4, 0);
    bb.write_rf(path, 0x0, 0xf_0000, 0x3); // RR_MOD
    bb.write_rf(path, 0x5, 0x1, 1); // RR_RSV1 RST
    bb.write_rf(path, 0x10005, 0x1, 1); // RR_BBDC SEL
}

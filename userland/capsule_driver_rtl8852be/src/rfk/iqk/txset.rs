// NONOS Operating System (AGPL-3.0-or-later)

//! IQK's TX setup (Linux v6.12 rtw89 `rtw8852b_rfk.c`):
//! `_iqk_txclk_setting` (whose shape `_iqk_rxclk_setting` shares),
//! `_lok_res_table` and `_iqk_txk_setting`.

use crate::phy::bb::Bb;
use crate::phy::rf::RFREG_MASK;
use crate::regs::Mmio;

const R_ANAPAR_PW15: u32 = 0x30c;
const R_ANAPAR: u32 = 0x32c;

/// `_iqk_txclk_setting` (and the opening of `_iqk_rxclk_setting`, which
/// differs only in its values).
pub fn clk(bb: &Bb<'_, impl Mmio>, pw15: [u32; 2], anapar: [u32; 2]) {
    bb.mask(0x12b8, 0x4000_0000, 1); // R_P0_NRBW DBG
    bb.mask(0x32b8, 0x4000_0000, 1); // R_P1_DBGMOD ON
    let _ = bb.read32(0x32b8);
    bb.mask(R_ANAPAR_PW15, 0xff00_0000, pw15[0]);
    let _ = bb.read32(R_ANAPAR_PW15);
    bb.mask(R_ANAPAR_PW15, 0xff00_0000, pw15[1]);
    bb.mask(R_ANAPAR, 0xffff_0000, anapar[0]);
    let _ = bb.read32(R_ANAPAR);
    bb.mask(R_ANAPAR, 0xffff_0000, anapar[1]);
}

pub fn res_table(bb: &Bb<'_, impl Mmio>, path: u8, five: bool, ibias: u32) {
    bb.write_rf(path, 0xef, RFREG_MASK, 0x2); // RR_LUTWE
    bb.write_rf(path, 0x33, RFREG_MASK, five as u32); // RR_LUTWA
    bb.write_rf(path, 0x3f, RFREG_MASK, ibias); // RR_LUTWD0
    bb.write_rf(path, 0xef, RFREG_MASK, 0);
    bb.write_rf(path, 0x7c, 0x20, 1); // RR_TXVBUF DACEN
}

pub fn txk_setting(bb: &Bb<'_, impl Mmio>, path: u8, five: bool) {
    if five {
        bb.write_rf(path, 0x85, 0x3, 0); // RR_XGLNA2 SW
        bb.write_rf(path, 0x60, 0x7, 1); // RR_BIASA A
    } else {
        bb.write_rf(path, 0x90, 0x300, 0); // RR_XALNA2 SW2
        bb.write_rf(path, 0x51, 0x8_0000, 0); // RR_TXG1 ATT2
        bb.write_rf(path, 0x51, 0x800, 0); // ATT1
        bb.write_rf(path, 0x52, 0x800, 1); // RR_TXG2 ATT0
    }
    bb.write_rf(path, 0x55, 0x1f, 0); // RR_TXGA LOK_EXT
    bb.write_rf(path, 0xef, 0x4, 1); // RR_LUTWE LOK
    bb.write_rf(path, 0x33, 0xff, if five { 0x80 } else { 0 }); // RR_LUTWA M1
    bb.write_rf(path, 0x0, 0xf_fff0, 0x403e); // RR_MOD IQK
    let _ = bb.read_rf(path, 0x0, RFREG_MASK);
}

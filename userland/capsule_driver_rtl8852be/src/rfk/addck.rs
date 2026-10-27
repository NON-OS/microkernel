// NONOS Operating System (AGPL-3.0-or-later)

//! The first half of `_dac_cal` (Linux v6.12 rtw89 `rtw8852b_rfk.c`):
//! `_addck` with its backup and reload (the ADC DC offset of each path,
//! measured by the hardware and then forced as a manual value).

use super::reg5::{run, List};
use super::DackInfo;
use crate::phy::bb::Bb;
use crate::regs::Mmio;
use crate::wait::until_us;

const R_ADDCK: [u32; 2] = [0xc0f4, 0xc1f4];
const R_ADDCKR: [u32; 2] = [0xc0fc, 0xc1fc];
const R_ADDCKD: [u32; 2] = [0xc0f0, 0xc1f0];
const R_SAMPL_DLY: [u32; 2] = [0xc0d4, 0xc1d4];
/// R_P0_NRBW and R_P1_DBGMOD, each with its debug bit 30.
pub const R_DBG: [u32; 2] = [0x12b8, 0x32b8];
const R_ANAPAR: u32 = 0x32c;
const R_ANAPAR_PW15: u32 = 0x30c;
const ADCCLK: u32 = 0x4000_0000;
const FLTRST: u32 = 0x40_0000;
const ANAPAR_EN: u32 = 0x1_0000;
const PW15_H: u32 = 0xf00_0000;

/// `_check_addc`: the list, then 100 reads rtw89 averages for its log.
fn check_addc(bb: &Bb<'_, impl Mmio>, path: usize) {
    run(bb, if path == 0 { List::CheckAddcA } else { List::CheckAddcB });
    for _ in 0..100 {
        let _ = bb.read32(0x1730); // R_DBG32_D
    }
}

pub fn addck(bb: &Bb<'_, impl Mmio>, d: &mut DackInfo) {
    bb.mask(R_ADDCK[0], 0x30, 0); // S0 B_ADDCK0_MAN
    bb.mask(R_SAMPL_DLY[1], 0x30, 0);
    for path in 0..2 {
        bb.mask(R_DBG[path], 0x4000_0000, 1);
        bb.mask(R_ANAPAR, ADCCLK, 0);
        bb.mask(R_ANAPAR, FLTRST, 0);
        bb.mask(R_ANAPAR, FLTRST, 1);
        bb.mask(R_ANAPAR_PW15, PW15_H, 0xf);
        bb.mask(R_ANAPAR, ANAPAR_EN, 0);
        bb.mask(R_SAMPL_DLY[path], 0x2, 1);
        bb.mask(R_ANAPAR_PW15, PW15_H, 0x3);
        check_addc(bb, path);
        bb.mask(R_ADDCK[path], 0x800, 1); // TRG
        bb.mask(R_ADDCK[path], 0x800, 0);
        let _ = bb.read32(R_ADDCK[path]); // rtw89's udelay(1)
        bb.mask(R_ADDCK[path], 0x300, 1);
        d.addck_timeout[path] = !until_us(10_000, || bb.get(R_ADDCKR[path], 0x1) != 0);
        check_addc(bb, path);
        bb.mask(R_SAMPL_DLY[path], 0x2, 0);
        bb.mask(R_ANAPAR, ANAPAR_EN, 1);
        bb.mask(R_ANAPAR_PW15, PW15_H, 0xc);
        bb.mask(R_ANAPAR, ADCCLK, 1);
        bb.mask(R_DBG[path], 0x4000_0000, 0);
    }
}

pub fn addck_backup_reload(bb: &Bb<'_, impl Mmio>, d: &mut DackInfo) {
    for path in 0..2 {
        bb.mask(R_ADDCK[path], 0x300, 0);
        d.addck_d[path] = [bb.get(R_ADDCKR[path], 0xf_fc00), bb.get(R_ADDCKR[path], 0x3ff)];
    }
    for path in 0..2 {
        let [i, q] = d.addck_d[path];
        bb.mask(R_ADDCKD[path], 0x3ff_0000, i);
        bb.mask(R_ADDCK[path], 0xf, q >> 6);
        bb.mask(R_ADDCKD[path], 0xfc00_0000, q & 0x3f);
        bb.mask(R_ADDCK[path], 0x30, 0x3);
    }
}

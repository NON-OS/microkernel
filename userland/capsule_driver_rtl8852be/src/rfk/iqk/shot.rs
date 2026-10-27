// NONOS Operating System (AGPL-3.0-or-later)

//! One IQK step: `_iqk_one_shot` and `_iqk_check_cal` (Linux v6.12 rtw89
//! `rtw8852b_rfk.c`). The command goes to the NCTL block, which reports
//! 0x55 in 0xbff8 when finished (within 8.2 ms); after 200 us the report's
//! flag bit says whether the step failed. A step that never finished
//! counts as failed.

use crate::phy::bb::Bb;
use crate::regs::Mmio;
use crate::wait::{delay_ms, until_us};

pub const R_P0_RFCTM: u32 = 0x5864;
const RFCTM_EN: u32 = 0x2000_0000;
pub const R_NCTL_CFG: u32 = 0x8000;
pub const R_NCTL_RPT: u32 = 0x8008;
pub const R_NCTL_N1: u32 = 0x8010;
const R_IQK_DIF4: u32 = 0x802c;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shot {
    FlokCoarse,
    FlokFine,
    FlokVbuffer,
    /// TX IQK, with the bandwidth (0 for 20 MHz).
    Txk(u8),
    /// RX IQK, with the bandwidth.
    Rxk(u8),
}

/// The NCTL command word, and the RFCTM enable the step runs with.
pub const fn command(s: Shot, path: u8) -> (u32, bool) {
    let p = 1 << (4 + path);
    match s {
        Shot::FlokCoarse => (0x108 | p, true),
        Shot::FlokFine => (0x208 | p, true),
        Shot::FlokVbuffer => (0x308 | p, true),
        Shot::Txk(bw) => (0x008 | p | (((0x8 + bw as u32) & 0xf) << 8), false),
        Shot::Rxk(bw) => (0x008 | p | (((0xb + bw as u32) & 0xf) << 8), true),
    }
}

/// `_iqk_check_cal`: true when the step failed.
pub fn check_cal(bb: &Bb<'_, impl Mmio>) -> bool {
    let done = until_us(8200, || bb.get(0xbff8, 0xff) == 0x55);
    delay_ms(1); // udelay(200)
    let fail = !done || bb.get(R_NCTL_RPT, 0x400_0000) != 0;
    bb.mask(R_NCTL_N1, 0xff, 0);
    fail
}

/// `_iqk_one_shot`: true when the step failed.
pub fn one_shot(bb: &Bb<'_, impl Mmio>, path: u8, s: Shot) -> bool {
    let (cmd, rfctm) = command(s, path);
    bb.mask(R_P0_RFCTM, RFCTM_EN, rfctm as u32);
    bb.mask(R_NCTL_CFG, u32::MAX, cmd + 1);
    let _ = bb.read32(R_NCTL_CFG); // rtw89's udelay(1); see `rf::write_rf`
    let fail = check_cal(bb);
    bb.mask(R_P0_RFCTM, RFCTM_EN, 0);
    fail
}

/// The TX tone setting `_iqk_lok` writes before its coarse and fine
/// steps (`R_IQK_DIF4`, TXT).
pub fn dif4_txt(bb: &Bb<'_, impl Mmio>, v: u32) {
    bb.mask(R_IQK_DIF4, 0xfff, v);
}

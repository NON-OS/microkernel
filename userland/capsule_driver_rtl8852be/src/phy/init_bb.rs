// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_phy_init_bb_reg` (Linux v6.12 rtw89 `phy.c`): the BB table written
//! (`rtw89_phy_config_bb_reg`: 0xf9-0xfe are delays, 0xbabecafe is skipped),
//! the TX power unit set (`__rtw8852bx_init_txpwr_unit`), the BB gain table
//! read into `BbGain`, then the 8852B's BB reset (`rtw8852b_bb_reset`).

use super::bb::Bb;
use super::gain::BbGain;
use super::reset::bb_reset;
use super::table::{walk, TableFail};
use crate::regs::{Mmio, Rw};
use crate::wait::delay_ms;

pub const BB_TABLE: &[u8] = include_bytes!("../../tables/bb.bin");
pub const BB_GAIN_TABLE: &[u8] = include_bytes!("../../tables/bb_gain.bin");
pub const BYPASS_CR_DATA: u32 = 0xbabe_cafe;

/// The delay, in whole milliseconds, a flow-control entry asks for.
pub const fn delay_of(addr: u32) -> Option<u64> {
    match addr {
        0xfe => Some(50),
        0xfd => Some(5),
        0xf9..=0xfc => Some(1),
        _ => None,
    }
}

pub fn init_bb(r: &impl Mmio, rfe: u8, cv: u8, gain: &mut BbGain) -> Result<(), TableFail> {
    let bb = Bb(r);
    walk(BB_TABLE, rfe, cv, |a, d| match delay_of(a) {
        Some(ms) => delay_ms(ms),
        None if d == BYPASS_CR_DATA => {}
        None => bb.write32(a, d),
    })?;
    txpwr_unit(r);
    walk(BB_GAIN_TABLE, rfe, cv, |a, d| gain.take(a, d))?;
    bb_reset(r);
    Ok(())
}

/// `__rtw8852bx_init_txpwr_unit` for PHY 0, with its UL TB offset of 0.
pub fn txpwr_unit(r: &impl Mmio) {
    r.write32(0xd248, 0x0776_3333); // PWR_UL_CTRL2
    r.write32(0xd220, 0x01eb_f000); // PWR_COEXT_CTRL
    r.write32(0xd240, 0x0002_f8ff); // PWR_UL_CTRL0
    r.set32(0xd288, 0x8000_0000); // PWR_UL_TB_CTRL: enable
    r.mask32(0xd28c, 0x1f, 0); // 1T offset 0
    r.mask32(0xd290, 0x1f, (-3i32) as u32); // 2T: max(0 - 3, -16)
}

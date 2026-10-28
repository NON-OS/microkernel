// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw89_phy_init_rf_nctl` (Linux v6.12 rtw89 `phy.c`): the IQK/DPK
//! block's clocks and resets (`rtw89_phy_preinit_rf_nctl_ax`), a poll of
//! its ready word (0x8080 reads 4 within 1 ms), then the NCTL table, which
//! rtw89 writes as BB registers (`rtw89_phy_config_bb_reg`).

use super::bb::Bb;
use super::init_bb::{delay_of, BYPASS_CR_DATA};
use super::table::{walk, TableFail};
use crate::regs::Mmio;
use crate::wait::{delay_ms, until_us};

pub const NCTL_TABLE: &[u8] = include_bytes!("../../tables/nctl.bin");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NctlFail {
    /// The block never answered 4 ("failed to poll nctl block"); rtw89
    /// logs it and goes on, this driver stops: every calibration needs it.
    NotReady,
    Table(TableFail),
}

pub fn init_rf_nctl(r: &impl Mmio, rfe: u8, cv: u8) -> Result<(), NctlFail> {
    let bb = Bb(r);
    bb.set(0xc60, 0x3); // R_IOQ_IQK_DPK
    bb.set(0xc6c, 0x1); // R_GNT_BT_WGT_EN
    bb.set(0x58ac, 0x800_0000); // R_P0_PATH_RST
    bb.set(0x78ac, 0x800_0000); // R_P1_PATH_RST
    bb.set(0xc60, 0x2); // the 8852B's second IQK/DPK set
    bb.write32(0x8000, 0x8); // R_NCTL_CFG
    let ready = until_us(1000, || {
        bb.write32(0x8080, 0x4);
        let _ = bb.read32(0x8080); // rtw89's udelay(1); see `rf::write_rf`
        bb.read32(0x8080) == 0x4
    });
    if !ready {
        return Err(NctlFail::NotReady);
    }
    walk(NCTL_TABLE, rfe, cv, |a, d| match delay_of(a) {
        Some(ms) => delay_ms(ms),
        None if d == BYPASS_CR_DATA => {}
        None => bb.write32(a, d),
    })
    .map_err(NctlFail::Table)
}

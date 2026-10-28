// NONOS Operating System (AGPL-3.0-or-later)

//! The PHY brought up, in the order of rtw88 `rtw8822c_phy_set_param`
//! (rtw8822c.c:1863): BB and RF powered, the header init around the tables,
//! the crystal, both paths set to receive (and path A to send one stream),
//! the CCK gain bounds the receive signal is read against, then
//! `rtw8822c_rf_init`: the DAC calibration and the factory trims.
//! rtw88's power tracking, CFO tracking, beamforming and DIG state are run
//! time decisions this driver does not make; what they write later is not
//! written here either.

use super::bits::{read32_mask, write32_mask};
use super::cond::{PhyCond, INTF_PCIE};
use super::trx::{BB_PATH_A, BB_PATH_AB};
use super::{apply, dack, power, rf, trim, trx};
use crate::efuse::EfuseInfo;
use crate::regs::Mmio;

/// `REG_ANAPAR_XTAL_0` and its crystal capacitance field (both halves).
const REG_ANAPAR_XTAL_0: usize = 0x1040;
const XTAL_CAP_MASK: u32 = 0x00FF_FC00;

/// The front ends rtw88 has a definition for (`rtw8822c_rfe_defs`, 0 to 6);
/// it refuses any other rather than program the nearest.
pub fn rfe_is_supported(rfe: u8) -> bool {
    rfe <= 6
}

/// The condition the tables are matched against (`rtw_phy_setup_phy_cond`):
/// a cut of 0 and the 8822C's package (none) count as 15.
pub fn cond(info: &EfuseInfo) -> PhyCond {
    let cut = if info.cut != 0 { info.cut } else { 15 };
    PhyCond { cut, pkg: 15, intf: INTF_PCIE, rfe: info.rfe }
}

/// The crystal capacitance word: the 7-bit value in both halves of the field.
pub fn xtal_word(xtal: u8) -> u32 {
    let cap = u32::from(xtal & 0x7F);
    cap | (cap << 7)
}

/// The CCK gain index bounds (upper, lower) from the AGC tables just loaded.
pub fn cck_gain_bounds<M: Mmio>(mmio: &M) -> (u8, u8) {
    let u = (read32_mask(mmio, 0x1A98, 0xC000) << 4) | read32_mask(mmio, 0x1AA8, 0xF_0000);
    let l = (read32_mask(mmio, 0x1A98, 0xC0) << 4) | read32_mask(mmio, 0x1A70, 0x0F00_0000);
    (u as u8, l as u8)
}

/// Bring the PHY up for the board `info` describes. False when the DAC
/// calibration's results did not read back (it still ran to its end).
pub fn bring_up<M: Mmio>(mmio: &M, info: &EfuseInfo) -> bool {
    rf::set_paths(info.paths);
    let paths = rf::paths();
    power::power_on(mmio);
    power::header_init(mmio, true);
    apply::load_all(mmio, &cond(info), paths);
    write32_mask(mmio, REG_ANAPAR_XTAL_0, XTAL_CAP_MASK, xtal_word(info.xtal));
    power::header_init(mmio, false);
    let ant = if paths == 2 { BB_PATH_AB } else { BB_PATH_A };
    trx::config(mmio, ant, ant);
    let (u, l) = cck_gain_bounds(mmio);
    crate::rx::signal::set_cck_bounds(u, l);
    let dack = dack::run(mmio);
    trim::x2_check(mmio);
    trim::thermal(mmio, &info.trim, paths);
    trim::power(mmio, &info.trim, paths);
    trim::pa_bias(mmio, &info.trim, paths);
    dack
}

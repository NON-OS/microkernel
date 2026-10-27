// NONOS Operating System (AGPL-3.0-or-later)

//! Read the board facts out of the efuse: dump the physical map, unpack the
//! logical one, take the facts (`facts`), wipe both maps. The cut version and
//! the RF path count are the chip's, read from `REG_SYS_CFG1` as rtw88
//! `rtw_chip_parameter_setup` reads them.

use nonos_wifi_core::vault::wipe;

use super::diag::record_all_ones;
use super::facts::{facts, PathPower, Trim};
use super::layout::*;
use super::{deshuffle, dump_physical};
use crate::regs::Mmio;

/// `BIT_RF_TYPE_ID` in `REG_SYS_CFG1`: the chip has two RF paths (2T2R).
const RF_TYPE_ID: u32 = 1 << 27;

/// The board facts the PHY bring-up needs.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct EfuseInfo {
    /// The cut (chip revision), `REG_SYS_CFG1` bits 12..15.
    pub cut: u8,
    /// RF paths: 2 on a 2T2R chip, else 1.
    pub paths: u8,
    /// The RF front-end option, the whole byte (`map->rfe_option`).
    pub rfe: u8,
    /// The crystal capacitance (`xtal_k & XCAP_MASK`).
    pub xtal: u8,
    /// Path A's and path B's 2.4 GHz power indexes.
    pub power: [PathPower; 2],
    /// The factory trims.
    pub trim: Trim,
}

/// The facts, or `None` when the efuse never read, read all ones, did not
/// unpack, or carries no front end. Neither map outlives this call.
pub fn read<M: Mmio>(mmio: &M) -> Option<EfuseInfo> {
    let mut phys = dump_physical(mmio)?;
    if phys.iter().all(|&b| b == 0xFF) {
        record_all_ones();
        return None;
    }
    let mut log = [0xFFu8; LOG_SIZE];
    let got = if deshuffle(&phys, &mut log) { facts(&phys, &log) } else { None };
    wipe(&mut phys);
    wipe(&mut log);
    let (rfe, xtal, power, trim) = got?;
    let cfg = mmio.read32(REG_CHIP_VER);
    let cut = ((cfg >> 12) & 0xF) as u8;
    let paths = if cfg & RF_TYPE_ID != 0 { 2 } else { 1 };
    Some(EfuseInfo { cut, paths, rfe, xtal, power, trim })
}

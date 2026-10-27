// NONOS Operating System (AGPL-3.0-or-later)

//! Powering the MAC on and off: `rtw89_mac_power_switch` (Linux v6.12 rtw89
//! `mac.c`) over the 8852B's own sequences (`rtw8852b.c`,
//! `rtw8852b_pwr_on_func` / `_off_func`; the chip has no table sequence).

mod off;
mod on;
mod on_rails;
pub mod regs;
pub mod xtal;

use crate::regs::{get, Mmio};
use regs::*;

/// What the power sequence needs of the board. Before the efuse is read
/// rtw89 knows none of it (`efuse.valid` false), as on the first power-on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Board {
    pub rfe_type: Option<u8>,
    pub efuse_valid: bool,
    pub power_k_valid: bool,
    pub cv: u8,
}

/// The step that did not complete.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PwrFail {
    /// Asked on while the MAC already reads powered (rtw89's -EBUSY).
    AlreadyOn,
    SysPower,
    AonOff,
    OnMac,
    OffMac,
    Xtal,
}

/// `rtw89_mac_power_switch`: refuse to power on a MAC already on, run the
/// chip's sequence, and leave the scoreboard note rtw89 leaves.
pub fn switch(r: &impl Mmio, on: bool, board: &Board) -> Result<(), PwrFail> {
    if on && get(r.read32(R_AX_IC_PWR_STATE), B_AX_WLMAC_PWR_STE_MASK) == PWR_ACT {
        return Err(PwrFail::AlreadyOn);
    }
    if on {
        on::power_on(r, board)?;
    } else {
        off::power_off(r, board)?;
    }
    let note = if on { MAC_AX_NOTIFY_TP_MAJOR } else { MAC_AX_NOTIFY_PWR_MAJOR };
    r.write8(R_AX_SCOREBOARD + 3, note);
    Ok(())
}

/// `rtw89_mac_partial_init`'s power step: on, and when that fails (a MAC a
/// warm reboot left on reads "already on"), off and on once more.
pub fn power_up(r: &impl Mmio, board: &Board) -> Result<(), PwrFail> {
    if switch(r, true, board).is_ok() {
        return Ok(());
    }
    let _ = switch(r, false, board);
    switch(r, true, board)
}

/// `rtw89_read_chip_ver`: the cut from SYS_CFG1, and (8852B being an
/// "885xb" chip) the analog cut from the crystal block.
pub fn chip_ver(r: &impl Mmio) -> (u8, Option<u8>) {
    let cv = get(r.read32(R_AX_SYS_CFG1), B_AX_CHIP_VER_MASK) as u8;
    let acv = xtal::read(r, xtal::XTAL_SI_CV).ok().map(|v| v & 0xf);
    (cv, acv)
}

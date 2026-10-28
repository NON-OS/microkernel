// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8852b_pwr_off_func` (Linux v6.12 rtw89 `rtw8852b.c`): the RF front
//! end and analog rails down through the crystal block, the baseband held
//! in reset, the MAC asked off, and the chip left in software low power.

use super::on::{poll, sps_ana};
use super::regs::*;
use super::xtal::{self, *};
use super::{Board, PwrFail};
use crate::regs::{Mmio, Rw};

/// The crystal writes before the MAC is asked off: (offset, value, mask).
pub const DOWN_SEQ: [(u8, u8, u8); 8] = [
    (XTAL_SI_ANAPAR_WL, RFC2RF, RFC2RF),
    (XTAL_SI_ANAPAR_WL, 0, OFF_EI),
    (XTAL_SI_ANAPAR_WL, 0, OFF_WEI),
    (XTAL_SI_WL_RFC_S0, 0, RF00),
    (XTAL_SI_WL_RFC_S1, 0, RF00),
    (XTAL_SI_ANAPAR_WL, SRAM2RFC, SRAM2RFC),
    (XTAL_SI_ANAPAR_WL, 0, PON_EI),
    (XTAL_SI_ANAPAR_WL, 0, PON_WEI),
];

pub fn power_off(r: &impl Mmio, board: &Board) -> Result<(), PwrFail> {
    sps_ana(r, board);
    for (off, val, mask) in DOWN_SEQ {
        xtal::write(r, off, val, mask).map_err(|_| PwrFail::Xtal)?;
    }
    r.set32(R_AX_SYS_PW_CTRL, B_AX_EN_WLON);
    r.clr32(R_AX_WLRF_CTRL, B_AX_AFC_AFEDIG);
    r.clr8(R_AX_SYS_FUNC_EN, B_AX_FEN_BB_GLB_RSTN | B_AX_FEN_BBRSTB);
    r.clr32(R_AX_SYS_ADIE_PAD_PWR_CTRL, B_AX_SYM_PADPDN_WL_RFC_1P3);
    xtal::write(r, XTAL_SI_ANAPAR_WL, 0, SHDN_WL).map_err(|_| PwrFail::Xtal)?;
    r.clr32(R_AX_SYS_ADIE_PAD_PWR_CTRL, B_AX_SYM_PADPDN_WL_PTA_1P3);
    xtal::write(r, XTAL_SI_ANAPAR_WL, 0, GND_SHDN_WL).map_err(|_| PwrFail::Xtal)?;
    r.set32(R_AX_SYS_PW_CTRL, B_AX_APFM_OFFMAC);
    poll(r, R_AX_SYS_PW_CTRL, B_AX_APFM_OFFMAC, false, PwrFail::OffMac)?;
    r.write32(R_AX_WLLPS_CTRL, SW_LPS_OPTION);
    r.set32(R_AX_SYS_SWR_CTRL1, B_AX_SYM_CTRL_SPS_PWMFREQ);
    r.mask32(R_AX_SPS_DIG_ON_CTRL0, B_AX_REG_ZCDC_H_MASK, 0x3);
    r.set32(R_AX_SYS_PW_CTRL, B_AX_APFM_SWLPS);
    Ok(())
}

// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8852b_pwr_on_func`, its second half: the WL analog rails through the
//! crystal block, the isolation released, the regulator trim a board with
//! a valid efuse and no power calibration takes, and the DMAC and CMAC
//! function enables.

use super::regs::*;
use super::xtal::{self, *};
use super::{Board, PwrFail};
use crate::regs::{Mmio, Rw};
use crate::wait::delay_ms;

/// The crystal writes in rtw89's order: (offset, value, mask).
pub const RAIL_SEQ: [(u8, u8, u8); 10] = [
    (XTAL_SI_ANAPAR_WL, SHDN_WL, SHDN_WL),
    (XTAL_SI_ANAPAR_WL, OFF_WEI, OFF_WEI),
    (XTAL_SI_ANAPAR_WL, OFF_EI, OFF_EI),
    (XTAL_SI_ANAPAR_WL, 0, RFC2RF),
    (XTAL_SI_ANAPAR_WL, PON_WEI, PON_WEI),
    (XTAL_SI_ANAPAR_WL, PON_EI, PON_EI),
    (XTAL_SI_ANAPAR_WL, 0, SRAM2RFC),
    (XTAL_SI_SRAM_CTRL, 0, SRAM_DIS),
    (XTAL_SI_XTAL_XMD_2, 0, LDO_LPS),
    (XTAL_SI_XTAL_XMD_4, 0, LPS_CAP),
];

pub const DMAC_FUNC: u32 = 0x7fff_8000;
pub const CMAC_FUNC: u32 = 0x7000_803f;

pub fn rails_and_func(r: &impl Mmio, board: &Board) -> Result<(), PwrFail> {
    r.set32(R_AX_SYS_ADIE_PAD_PWR_CTRL, B_AX_SYM_PADPDN_WL_PTA_1P3);
    xtal::write(r, XTAL_SI_ANAPAR_WL, GND_SHDN_WL, GND_SHDN_WL).map_err(|_| PwrFail::Xtal)?;
    r.set32(R_AX_SYS_ADIE_PAD_PWR_CTRL, B_AX_SYM_PADPDN_WL_RFC_1P3);
    for (off, val, mask) in RAIL_SEQ {
        xtal::write(r, off, val, mask).map_err(|_| PwrFail::Xtal)?;
    }
    r.set32(R_AX_PMC_DBG_CTRL2, B_AX_SYSON_DIS_PMCR_AX_WRMSK);
    r.set32(R_AX_SYS_ISO_CTRL, B_AX_ISO_EB2CORE);
    r.clr32(R_AX_SYS_ISO_CTRL, B_AX_PWC_EV2EF_B15);
    delay_ms(1);
    r.clr32(R_AX_SYS_ISO_CTRL, B_AX_PWC_EV2EF_B14);
    r.clr32(R_AX_PMC_DBG_CTRL2, B_AX_SYSON_DIS_PMCR_AX_WRMSK);
    if board.efuse_valid && !board.power_k_valid {
        r.mask32(R_AX_SPS_DIG_ON_CTRL0, B_AX_VOL_L1_MASK, 0x9);
        r.mask32(R_AX_SPS_DIG_ON_CTRL0, B_AX_VREFPFM_L_MASK, 0xa);
        if board.cv == CHIP_CBV {
            r.set32(R_AX_PMC_DBG_CTRL2, B_AX_SYSON_DIS_PMCR_AX_WRMSK);
            r.mask16(R_AX_HCI_LDO_CTRL, B_AX_R_AX_VADJ_MASK, 0xa);
            r.clr32(R_AX_PMC_DBG_CTRL2, B_AX_SYSON_DIS_PMCR_AX_WRMSK);
        }
    }
    r.set32(crate::mac::R_AX_DMAC_FUNC_EN, DMAC_FUNC);
    r.set32(crate::mac::R_AX_CMAC_FUNC_EN, CMAC_FUNC);
    let sel = PINMUX_EESK_FUNC_SEL_BT_LOG;
    r.mask32(R_AX_EECS_EESK_FUNC_SEL, B_AX_PINMUX_EESK_FUNC_SEL_MASK, sel);
    Ok(())
}

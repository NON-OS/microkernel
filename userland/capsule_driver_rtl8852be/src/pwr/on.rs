// NONOS Operating System (AGPL-3.0-or-later)

//! `rtw8852b_pwr_on_func` (Linux v6.12 rtw89 `rtw8852b.c`), its first half:
//! the always-on block out of suspend, the analog rails up, the MAC asked
//! on, and the platform reset pulsed. Each poll is rtw89's 20 ms.

use super::on_rails::rails_and_func;
use super::regs::*;
use super::{Board, PwrFail};
use crate::regs::{Mmio, Rw};
use crate::wait::until_us;

const POLL_US: u64 = 20_000;

pub fn power_on(r: &impl Mmio, board: &Board) -> Result<(), PwrFail> {
    sps_ana(r, board);
    r.clr32(R_AX_SYS_PW_CTRL, B_AX_AFSM_WLSUS_EN | B_AX_AFSM_PCIE_SUS_EN);
    r.set32(R_AX_SYS_PW_CTRL, B_AX_DIS_WLBT_PDNSUSEN_SOPC);
    r.set32(R_AX_WLLPS_CTRL, B_AX_DIS_WLBT_LPSEN_LOPC);
    r.clr32(R_AX_SYS_PW_CTRL, B_AX_APDM_HPDN);
    r.clr32(R_AX_SYS_PW_CTRL, B_AX_APFM_SWLPS);
    poll(r, R_AX_SYS_PW_CTRL, B_AX_RDY_SYSPWR, true, PwrFail::SysPower)?;
    r.set32(R_AX_AFE_LDO_CTRL, B_AX_AON_OFF_PC_EN);
    poll(r, R_AX_AFE_LDO_CTRL, B_AX_AON_OFF_PC_EN, true, PwrFail::AonOff)?;
    r.mask32(R_AX_SPS_DIG_OFF_CTRL0, B_AX_C1_L1_MASK, 0x1);
    r.mask32(R_AX_SPS_DIG_OFF_CTRL0, B_AX_C3_L1_MASK, 0x3);
    r.set32(R_AX_SYS_PW_CTRL, B_AX_EN_WLON);
    r.set32(R_AX_SYS_PW_CTRL, B_AX_APFN_ONMAC);
    poll(r, R_AX_SYS_PW_CTRL, B_AX_APFN_ONMAC, false, PwrFail::OnMac)?;
    // The platform pulse, twice, then left set: as rtw89 writes it.
    for _ in 0..2 {
        r.set8(R_AX_PLATFORM_ENABLE, B_AX_PLATFORM_EN);
        r.clr8(R_AX_PLATFORM_ENABLE, B_AX_PLATFORM_EN);
    }
    r.set8(R_AX_PLATFORM_ENABLE, B_AX_PLATFORM_EN);
    r.clr32(R_AX_SYS_SDIO_CTRL, B_AX_PCIE_CALIB_EN_V1);
    rails_and_func(r, board)
}

/// `rtw8852b_pwr_sps_ana`: RFE type 5 boards trim the switching regulator.
pub fn sps_ana(r: &impl Mmio, board: &Board) {
    if board.rfe_type == Some(5) {
        r.write16(R_AX_SPS_ANA_ON_CTRL2, RTL8852B_RFE_05_SPS_ANA);
    }
}

/// Wait for `bits` in `reg` to read set (`set`) or clear.
pub fn poll(r: &impl Mmio, reg: u32, bits: u32, set: bool, fail: PwrFail) -> Result<(), PwrFail> {
    let ok = until_us(POLL_US, || (r.read32(reg) & bits != 0) == set);
    ok.then_some(()).ok_or(fail)
}

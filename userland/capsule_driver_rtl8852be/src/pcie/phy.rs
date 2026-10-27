// NONOS Operating System (AGPL-3.0-or-later)

//! The first half of `rtw89_pci_ops_mac_pre_init_ax` (Linux v6.12 rtw89
//! `pci.c`) as it runs on the 8852B: of the steps there, the 8852A- and
//! 8852C-only ones and the BER quirk (`rtw8852be.c` sets no quirks) do
//! nothing on this chip and are left out; the rest run in rtw89's order.

use super::regs::*;
use super::{dbi, mdio, PcieFail};
use crate::regs::{field, Mmio, Rw};

/// `PCIE_DPHY_DLY_25US`, `PCIE_AUTOK_4`.
const DPHY_DLY_25US: u16 = 0x1;
const AUTOK_4: u16 = 0x3;

pub fn phy_setup(r: &impl Mmio) -> Result<(), PcieFail> {
    // rtw89_pci_l1off_pwroff, _aphy_pwrcut, _hci_ldo
    r.clr32(R_AX_PCIE_PS_CTRL, B_AX_L1OFF_PWR_OFF_EN);
    r.clr32(R_AX_SYS_PW_CTRL, B_AX_PSUS_OFF_CAPC_EN);
    r.set32(R_AX_SYS_SDIO_CTRL, B_AX_PCIE_DIS_L2_CTRL_LDO_HCI);
    r.clr32(R_AX_SYS_SDIO_CTRL, B_AX_PCIE_DIS_WLSUS_AFT_PDN);
    // rtw89_pci_dphy_delay, _autok_x
    let g1 = mdio::GEN1;
    mdio::write16_mask(r, RAC_REG_REV2, BAC_CMU_EN_DLY_MASK, DPHY_DLY_25US, g1)?;
    mdio::write16_mask(r, RAC_REG_FLD_0, BAC_AUTOK_N_MASK, AUTOK_4, g1)?;
    refclk_cal_off(r)?;
    // rtw89_pci_power_wake(true), _set_sic, _set_lbc, _set_dbg, _set_keep_reg
    r.set32(R_AX_HCI_OPT_CTRL, BIT_WAKE_CTRL);
    r.clr32(R_AX_PCIE_EXP_CTRL, B_AX_SIC_EN_FORCE_CLKREQ);
    let lbc = r.read32(R_AX_LBC_WATCHDOG);
    let lbc = (lbc & !B_AX_LBC_TIMER) | field(B_AX_LBC_TIMER, MAC_AX_LBC_TMR_2MS);
    let lbc = lbc | B_AX_LBC_FLAG | B_AX_LBC_EN;
    r.write32(R_AX_LBC_WATCHDOG, lbc);
    r.set32(R_AX_LBC_WATCHDOG, lbc);
    r.set32(R_AX_PCIE_DBG_CTRL, B_AX_ASFF_FULL_NO_STK | B_AX_EN_STUCK_DBG);
    r.set32(R_AX_PCIE_INIT_CFG1, B_AX_PCIE_TXRST_KEEP_REG | B_AX_PCIE_RXRST_KEEP_REG);
    Ok(())
}

/// `rtw89_pci_auto_refclk_cal(rtwdev, false)`: the reference clock
/// calibration turned off, with L1 held off around the PHY access.
fn refclk_cal_off(r: &impl Mmio) -> Result<(), PcieFail> {
    let rate = dbi::read8(r, RTW89_PCIE_PHY_RATE)? & 0x3;
    let speed = match rate {
        1 => mdio::GEN1,
        2 => mdio::GEN2,
        _ => return Err(PcieFail::LinkRate(rate)),
    };
    let l1 = dbi::read8(r, RTW89_PCIE_L1_CTRL)?;
    if l1 & RTW89_PCIE_BIT_L1 != 0 {
        dbi::write8(r, RTW89_PCIE_L1_CTRL, l1 & !RTW89_PCIE_BIT_L1)?;
    }
    let cal = mdio::read16(r, RAC_CTRL_PPR_V1, speed).map_err(PcieFail::from);
    let done = cal.and_then(|v| match v & B_AX_CALIB_EN {
        0 => Ok(()),
        _ => Ok(mdio::write16(r, RAC_CTRL_PPR_V1, v & !B_AX_CALIB_EN, speed)?),
    });
    // As rtw89: with L1 restored, the restore's answer is the step's answer.
    if l1 & RTW89_PCIE_BIT_L1 != 0 {
        return Ok(dbi::write8(r, RTW89_PCIE_L1_CTRL, l1)?);
    }
    done
}

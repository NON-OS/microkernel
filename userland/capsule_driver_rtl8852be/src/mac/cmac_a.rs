// NONOS Operating System (AGPL-3.0-or-later)

//! `cmac_init_ax`'s first steps (Linux v6.12 rtw89 `mac.c`) for CMAC 0:
//! `scheduler_init_ax`, `addr_cam_init_ax`, `rx_fltr_init_ax` (every frame
//! type to the host, rtw89's default filter options and PLCP checks),
//! `cca_ctrl_init_ax` with the 8852B's `_patch_dis_resp_chk`,
//! `nav_ctrl_init_ax` and `spatial_reuse_init_ax`.

use super::{cmac_on, MacFail};
use crate::regs::{Mmio, Rw};
use crate::wait::until_us;

pub const R_AX_PREBKF_CFG_0: u32 = 0xc338;
pub const R_AX_PREBKF_CFG_1: u32 = 0xc33c;
pub const R_AX_CCA_CFG_0: u32 = 0xc340;
pub const R_AX_SCH_EXT_CTRL: u32 = 0xc3fc;
pub const R_AX_CCA_CONTROL: u32 = 0xc390;
pub const R_AX_RSP_CHK_SIG: u32 = 0xcc00;
pub const R_AX_TRXPTCL_RESP_0: u32 = 0xcc04;
pub const R_AX_WMAC_NAV_CTL: u32 = 0xcc80;
pub const R_AX_PLCP_HDR_FLTR: u32 = 0xce04;
pub const R_AX_RX_FLTR_OPT: u32 = 0xce20;
pub const R_AX_CTRL_FLTR: u32 = 0xce24;
pub const R_AX_MGNT_FLTR: u32 = 0xce28;
pub const R_AX_DATA_FLTR: u32 = 0xce2c;
pub const R_AX_ADDR_CAM_CTRL: u32 = 0xce34;
pub const R_AX_RX_SR_CTRL: u32 = 0xce4a;
pub const R_AX_BSSID_SRC_CTRL: u32 = 0xce4b;
/// `RX_FLTR_FRAME_TO_HOST` for every subtype.
pub const TO_HOST: u32 = 0x5555_5555;
/// `DEFAULT_AX_RX_FLTR`: A1 match, BC, MC, UC and BC CAM match, power
/// management, FTM request, UID filter 3, beacon check.
pub const DEFAULT_RX_FLTR: u32 = 0x0300_44be;
pub const PLCP_CHECKS: u16 = 0x7f;
/// What `cca_ctrl_init_ax` sets and clears.
pub const CCA_SET: u32 = 0x7121_00ff;
pub const CCA_CLR: u32 = 0x8e1e_0100;
pub const RSP_CHK_NAV: u32 = 0x38_0000;
pub const RSP_CHK_CCA: u32 = 0x1f80_0000;

pub fn cmac_a(r: &impl Mmio) -> Result<(), MacFail> {
    cmac_on(r)?;
    r.mask32(R_AX_PREBKF_CFG_1, 0x7f, 0x47);
    r.set32(R_AX_SCH_EXT_CTRL, 0x2);
    r.clr32(R_AX_CCA_CFG_0, 0x20);
    r.mask32(R_AX_PREBKF_CFG_0, 0x1f, 0x18);
    let cam = r.read32(R_AX_ADDR_CAM_CTRL) | 0x7f_0000 | 0x100 | 0x1;
    r.write32(R_AX_ADDR_CAM_CTRL, cam);
    if !until_us(2000, || r.read16(R_AX_ADDR_CAM_CTRL) & 0x100 == 0) {
        return Err(MacFail::Step(2));
    }
    for reg in [R_AX_MGNT_FLTR, R_AX_CTRL_FLTR, R_AX_DATA_FLTR] {
        r.write32(reg, TO_HOST);
    }
    r.write32(R_AX_RX_FLTR_OPT, DEFAULT_RX_FLTR);
    r.write16(R_AX_PLCP_HDR_FLTR, PLCP_CHECKS);
    let cca = (r.read32(R_AX_CCA_CONTROL) | CCA_SET) & !CCA_CLR;
    r.write32(R_AX_CCA_CONTROL, cca);
    let sig = r.read32(R_AX_RSP_CHK_SIG) & !RSP_CHK_NAV;
    r.write32(R_AX_RSP_CHK_SIG, sig);
    let resp = r.read32(R_AX_TRXPTCL_RESP_0) & !RSP_CHK_CCA;
    r.write32(R_AX_TRXPTCL_RESP_0, resp);
    r.set32(R_AX_WMAC_NAV_CTL, 0x0403_0000);
    r.mask32(R_AX_WMAC_NAV_CTL, 0xff00, 0xc4);
    r.clr8(R_AX_RX_SR_CTRL, 0x1);
    r.set8(R_AX_BSSID_SRC_CTRL, 0x1);
    Ok(())
}

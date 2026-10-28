// NONOS Operating System (AGPL-3.0-or-later)

//! The register writes of `rtw89_phy_dm_init`'s first steps (Linux v6.12
//! rtw89 `phy.c`, with `rtw8852b_common.c`): `__rtw8852bx_bb_sethw`, the
//! DIG reset (`rtw89_phy_dig_reset`: the 8852B has no IGI control, so the
//! PD thresholds and the follow-PAGC switches), the CFO init (crystal cap
//! from the efuse, `rtw89_dcfo_comp_init` with hardware compensation) and
//! `rtw89_phy_edcca_init`. rtw89's statistics, environment monitor and
//! PHY status parsing set up software counters and reports this driver
//! does not keep.

use super::bb::Bb;
use super::state::Phy;
use crate::pwr::xtal::{self, XtalTimeout};
use crate::regs::{Mmio, Rw};

const R_AX_PWR_MACID_LMT_TABLE0: u32 = 0xd36c;
const R_AX_PWR_MACID_LMT_TABLE127: u32 = 0xd568;
const XTAL_SI_XTAL_SC_XI: u8 = 0x4;
const XTAL_SI_XTAL_SC_XO: u8 = 0x5;

pub fn sethw(r: &impl Mmio, p: &mut Phy) {
    let bb = Bb(r);
    bb.clr(0xd7c, 0x2); // R_P0_EN_SOUND_WO_NDP
    bb.clr(0x2d7c, 0x2); // R_P1_EN_SOUND_WO_NDP
    for a in (R_AX_PWR_MACID_LMT_TABLE0..=R_AX_PWR_MACID_LMT_TABLE127).step_by(4) {
        r.write32(a, 0);
    }
    p.efuse_gain.offset_base = bb.get(0x49b0, 0xff) as u8 as i8; // R_P0_RPL1
    p.efuse_gain.rssi_base = bb.get(0x4a00, 0xff) as u8 as i8; // R_P1_RPL1
}

/// The CCK PD lower bound `rtw89_phy_dig_dyn_pd_th(rssi_nolink, false)`
/// writes at init: the IGI RSSI is still 0, the 20 MHz under-region is
/// 16 + 7, so max(0 - 23, -18) - 110 = -128.
pub const CCK_PD_LOW: u32 = (-18i32 - 110) as u32;

pub fn dig_reset(r: &impl Mmio, cv: u8) {
    let bb = Bb(r);
    bb.mask(0x4860, 0x7c0, 0); // R_SEG0R_PD_V1 lower bound
    bb.mask(0x4860, 0x4000_0000, 0); // spatial reuse off
    if cv > 0 {
        // support_cckpd: every 8852B cut after CAV.
        bb.mask(0x4b74, 0x4000_0000, 0); // R_BMODE_PDTH_EN_V1
        bb.mask(0x4b64, 0xff00_0000, CCK_PD_LOW); // R_BMODE_PDTH_V1
    }
    for a in [0x46e8, 0x46ec, 0x47a8, 0x47ac] {
        bb.mask(a, 0x20, 0); // P0/P1 P20/S20 follow PAGC/UGC
    }
}

pub fn cfo_init(r: &impl Mmio, xtal_k: u8) -> Result<(), XtalTimeout> {
    let cap = xtal_k & 0x7f;
    xtal::write(r, XTAL_SI_XTAL_SC_XO, cap, 0xff)?;
    xtal::write(r, XTAL_SI_XTAL_SC_XI, cap, 0xff)?;
    let bb = Bb(r);
    bb.mask(0x4494, 0x2000_0000, 1); // R_DCFO_OPT valid
    bb.mask(0x4490, 0xf00_0000, 8); // R_DCFO_WEIGHT
    r.mask32(0xd248, 0x7, 6); // R_AX_PWR_UL_CTRL2 CFO
    Ok(())
}

pub fn edcca_init(r: &impl Mmio) {
    Bb(r).mask(0xc70, 0x3f0_0000, 0x29); // R_TX_COLLISION_T2R_ST
}

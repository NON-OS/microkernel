// NONOS Operating System (AGPL-3.0-or-later)

//! `dmac_init_ax` (Linux v6.12 rtw89 `mac.c`): the service memory plan, full
//! host flow control, the station scheduler (`sta_sch_init_ax`), the MPDU
//! processor (`mpdu_proc_init_ax`: received frames keep their FCS) and the
//! security engine (`sec_eng_init_ax`). The 8852B takes no preload
//! (`rtw89_mac_preload_init`) and no SS2F path patch.

use super::{mac_on, MacFail};
use crate::dle::{dle_init, hfc_init, quota};
use crate::regs::{Mmio, Rw};
use crate::wait::until_us;

pub const R_AX_SS_CTRL: u32 = 0x9e10;
pub const B_AX_SS_EN: u8 = 0x1;
pub const B_AX_SS_INIT_DONE_1: u32 = 0x8000_0000;
pub const B_AX_SS_WARM_INIT_FLG: u32 = 0x2000_0000;
pub const B_AX_SS_NONEMPTY_SS2FINFO_EN: u32 = 0x1000_0000;
pub const R_AX_MPDU_PROC: u32 = 0x9c00;
pub const B_AX_APPEND_FCS: u32 = 0x1;
pub const B_AX_A_ICV_ERR: u32 = 0x2;
pub const R_AX_ACTION_FWD0: u32 = 0x9c04;
pub const R_AX_TF_FWD: u32 = 0x9c14;
pub const R_AX_CUT_AMSDU_CTRL: u32 = 0x9c40;
pub const MPDU_PROC_ACT_FRWD: u32 = 0x02a9_5a95;
pub const MPDU_PROC_TF_FRWD: u32 = 0xaa55;
pub const MPDU_PROC_CUT_CTRL: u32 = 0x010e_05f0;
pub const R_AX_SEC_ENG_CTRL: u32 = 0x9d00;
/// The three cipher clocks, TX encrypt, RX decrypt, MC and BC decrypt.
pub const SEC_ENG_ON: u32 = 0x070f;
pub const B_AX_TX_PARTIAL_MODE: u32 = 0x800;
pub const R_AX_SEC_MPDU_PROC: u32 = 0x9d04;
pub const APPEND_ICV_MIC: u32 = 0x3;

pub fn dmac_init(r: &impl Mmio) -> Result<(), MacFail> {
    mac_on(r)?;
    dle_init(r, &quota::SCC, None).map_err(MacFail::Dle)?;
    mac_on(r)?;
    hfc_init(r, true, true).map_err(MacFail::Hfc)?;
    mac_on(r)?;
    let ss = r.read8(R_AX_SS_CTRL) | B_AX_SS_EN;
    r.write8(R_AX_SS_CTRL, ss);
    if !until_us(2000, || r.read32(R_AX_SS_CTRL) & B_AX_SS_INIT_DONE_1 != 0) {
        return Err(MacFail::Step(1));
    }
    r.set32(R_AX_SS_CTRL, B_AX_SS_WARM_INIT_FLG);
    r.clr32(R_AX_SS_CTRL, B_AX_SS_NONEMPTY_SS2FINFO_EN);
    mac_on(r)?;
    r.write32(R_AX_ACTION_FWD0, MPDU_PROC_ACT_FRWD);
    r.write32(R_AX_TF_FWD, MPDU_PROC_TF_FRWD);
    r.set32(R_AX_MPDU_PROC, B_AX_APPEND_FCS | B_AX_A_ICV_ERR);
    r.write32(R_AX_CUT_AMSDU_CTRL, MPDU_PROC_CUT_CTRL);
    mac_on(r)?;
    let sec = (r.read32(R_AX_SEC_ENG_CTRL) | SEC_ENG_ON) & !B_AX_TX_PARTIAL_MODE;
    r.write32(R_AX_SEC_ENG_CTRL, sec);
    let p = r.read32(R_AX_SEC_MPDU_PROC) | APPEND_ICV_MIC;
    r.write32(R_AX_SEC_MPDU_PROC, p);
    Ok(())
}

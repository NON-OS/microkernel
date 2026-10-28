// NONOS Operating System (AGPL-3.0-or-later)

//! Pausing and resuming the MAC's TX scheduler around a channel switch:
//! `rtw89_mac_stop_sch_tx` and `rtw89_mac_resume_sch_tx` (Linux v6.12
//! rtw89 `mac.c`). With the firmware up, rtw89 asks it through the
//! register mailbox (`rtw89_hw_sch_tx_en_h2c`), which answers with a TX
//! pause report.

use super::{cmac_on, MacFail};
use crate::h2c::reg::{msg, MsgFail, C2H_TX_PAUSE_RPT, FUNC_SCH_TX_EN};
use crate::h2c::Fw;
use crate::regs::{field, Mmio};

pub const R_AX_CTN_TXEN: u32 = 0xc348;
pub const TXEN_ALL: u16 = 0xffff;
/// `sizeof(struct rtw89_h2creg_sch_tx_en) - RTW89_H2CREG_HDR_LEN`.
const CONTENT_LEN: u32 = 8 - 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SchFail {
    Mac(MacFail),
    Msg(MsgFail),
}

fn set(r: &impl Mmio, fw: &mut Fw, en: u16, mask: u16) -> Result<(), SchFail> {
    cmac_on(r).map_err(SchFail::Mac)?;
    let w0 = field(0xffff_0000, en as u32);
    let w1 = field(0xffff, mask as u32); // band 0
    msg(r, &mut fw.mailbox, FUNC_SCH_TX_EN, [w0, w1, 0, 0], CONTENT_LEN, C2H_TX_PAUSE_RPT)
        .map(|_| ())
        .map_err(SchFail::Msg)
}

/// Stop every queue; returns what was enabled, for `resume`.
pub fn stop(r: &impl Mmio, fw: &mut Fw) -> Result<u16, SchFail> {
    let was = r.read16(R_AX_CTN_TXEN);
    set(r, fw, 0, TXEN_ALL)?;
    Ok(was)
}

pub fn resume(r: &impl Mmio, fw: &mut Fw, was: u16) -> Result<(), SchFail> {
    set(r, fw, was, TXEN_ALL)
}

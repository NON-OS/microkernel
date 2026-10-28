// NONOS Operating System (AGPL-3.0-or-later)

//! The MAC: the pre-init before the download, then (after it) the system
//! and TRX init, the port, and the receive filter.

pub mod chan;
pub mod cmac_a;
pub mod cmac_b;
pub mod dmac;
pub mod imr;
pub mod macid;
pub mod port;
pub mod pre;
pub mod sch;
pub mod sys;
pub mod trx;

use crate::dle::{DleFail, HfcFail};
use crate::regs::Mmio;

pub const R_AX_DMAC_FUNC_EN: u32 = 0x8400;
pub const R_AX_CMAC_FUNC_EN: u32 = 0xc000;
/// MAC_FUNC_EN and DMAC_FUNC_EN, what `rtw89_mac_check_mac_en_ax` wants.
pub const MAC_DMAC_ON: u32 = 0x6000_0000;
/// `B_AX_CMAC_EN`.
pub const CMAC_ON: u32 = 0x4000_0000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MacFail {
    /// The DMAC (or CMAC) reads off or dead where rtw89 checks it.
    Off,
    Dle(DleFail),
    Hfc(HfcFail),
    /// A step's ready bit never came: the step's number in `trx`.
    Step(u8),
}

/// `rtw89_mac_check_mac_en_ax` for the DMAC: refuses a dead read
/// (`RTW89_R32_EA` or all ones) or a DMAC not enabled.
pub fn mac_on(r: &impl Mmio) -> Result<(), MacFail> {
    on(r.read32(R_AX_DMAC_FUNC_EN), MAC_DMAC_ON)
}

/// The same check for CMAC 0.
pub fn cmac_on(r: &impl Mmio) -> Result<(), MacFail> {
    on(r.read32(R_AX_CMAC_FUNC_EN), CMAC_ON)
}

fn on(v: u32, want: u32) -> Result<(), MacFail> {
    let dead = v == 0xeaea_eaea || v == 0xdead_beef || v == u32::MAX;
    (!dead && v & want == want).then_some(()).ok_or(MacFail::Off)
}

// NONOS Operating System (AGPL-3.0-or-later)

//! The bring-up, in rtw89's order: the probe (partial init, efuse, PHY
//! capabilities, power off), then `rtw89_core_start`: the full MAC init,
//! the PHY tables, coexistence, and the DM init with its calibrations.

pub mod dm_init;
pub mod mac_init;
mod partial;
pub mod phy_init;
pub mod probe;
pub mod start;
pub mod up;

pub use partial::partial_init;

use crate::efuse::EfuseFail;
use crate::fw::FwFail;
use crate::h2c::reg::MsgFail;
use crate::h2c::send::SendFail;
use crate::mac::sch::SchFail;
use crate::mac::MacFail;
use crate::pcie::PcieFail;
use crate::phy::nctl::NctlFail;
use crate::phy::table::TableFail;
use crate::pwr::PwrFail;

/// Where bring-up stopped, and why.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    Power(PwrFail),
    Pcie(PcieFail),
    Mac(MacFail),
    Firmware(FwFail),
    Efuse(EfuseFail),
    Mailbox(MsgFail),
    Send(SendFail),
    Table(TableFail),
    Xtal,
    Coex,
    Nctl(NctlFail),
    Sch(SchFail),
    /// A join asked before any channel was tuned.
    NoChannel,
}

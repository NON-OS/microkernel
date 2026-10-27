// NONOS Operating System (AGPL-3.0-or-later)

//! The bring-up, in rtw89's order: the partial init (power, pre-inits,
//! download), the board read from the efuse, then the full MAC init.

mod partial;

pub use partial::partial_init;

use crate::fw::FwFail;
use crate::mac::MacFail;
use crate::pcie::PcieFail;
use crate::pwr::PwrFail;

/// Where bring-up stopped, and why.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    Power(PwrFail),
    Pcie(PcieFail),
    Mac(MacFail),
    Firmware(FwFail),
}

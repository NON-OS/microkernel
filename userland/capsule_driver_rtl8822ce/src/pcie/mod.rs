// NONOS Operating System (AGPL-3.0-or-later)

//! The PCIe link. The DBI window, the link held awake for bring-up, the
//! host's Link Control and power save's ASPM are the RTL8821CE's; `cfg` is
//! what rtw88 does for the 8822C alone.

#[path = "../../../capsule_driver_rtl8821ce/src/pcie/aspm.rs"]
mod aspm;
#[path = "../../../capsule_driver_rtl8821ce/src/pcie/awake.rs"]
mod awake;
mod cfg;
#[path = "../../../capsule_driver_rtl8821ce/src/pcie/dbi.rs"]
mod dbi;
#[path = "../../../capsule_driver_rtl8821ce/src/pcie/host.rs"]
mod host;

pub use aspm::*;
pub use awake::*;
pub use cfg::{after_mac, board, clock_delay_off};
pub use host::*;

// NONOS Operating System (AGPL-3.0-or-later)

//! The PHY tables loaded in rtw88's order (`rtw_phy_load_tables`): baseband,
//! AGC, the RF calibration init table, then the RF tables. The settle delays
//! the tables carry are the RTL8821CE's (`delay`), as phy.c defines them for
//! every chip; the 8822C's PHY-MAC table is empty, so nothing is loaded for it.

#[path = "../../../../capsule_driver_rtl8821ce/src/phy/apply/delay.rs"]
mod delay;
mod load;

pub use load::load_all;

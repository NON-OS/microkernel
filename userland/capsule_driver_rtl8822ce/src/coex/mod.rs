// NONOS Operating System (AGPL-3.0-or-later)

//! Wi-Fi and Bluetooth share the 8822CE's antennas. rtw88 hands them to
//! Wi-Fi in its coexistence setup after every power-on (coex.c and
//! rtw8822c.c); this driver runs no Bluetooth, so it takes the Wi-Fi-only
//! path. The LTE-coex indirect window is the RTL8821CE's (the same
//! registers, `rtw8822c_ltecoex_addr`); the 8822C has no antenna switch for
//! the driver to set (`ant_switch_exist` false in its rfe_type).

#[path = "../../../capsule_driver_rtl8821ce/src/coex/indirect.rs"]
mod indirect;
mod init;
mod wl_only;

pub use wl_only::take_antenna;

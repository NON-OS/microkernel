// NONOS Operating System (AGPL-3.0-or-later)

//! The RTL8822CE's baseband and two RF paths: power, the vendor tables, the
//! path configuration, the DAC calibration and trims (`setup`), the channel
//! (`channel`, the `set_rf` the shared serve loop retunes with) and transmit
//! power. The table condition parser is the RTL8821CE's (`cond`), rtw88's
//! `rtw_parse_tbl_phy_cond` for every chip.

pub mod apply;
pub mod bits;
pub mod channel;
#[path = "../../../capsule_driver_rtl8821ce/src/phy/cond.rs"]
pub mod cond;
pub mod dack;
pub mod igi;
pub mod power;
pub mod rf;
pub mod setup;
pub mod tables;
pub mod trim;
pub mod trx;
pub mod txpower;

pub use setup::bring_up;

// NONOS Operating System (AGPL-3.0-or-later)
//! The feature report logic as HID over SPI mounts it: without the I2C
//! port, so a modelled port can stand in.

#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/configure.rs"]
mod configure;
#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/configured.rs"]
mod configured;
#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/fallback.rs"]
mod fallback;
#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/one_report.rs"]
mod one_report;
#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/port.rs"]
pub mod port;
#[path = "../../../capsule_driver_i2c_hid/src/hid/input_mode/set_bits.rs"]
mod set_bits;

pub use configure::configure_reporting;

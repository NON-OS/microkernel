// NONOS Operating System (AGPL-3.0-or-later)
//! The broker's record types and the device-tree DesignWare I2C mapping.

#[allow(dead_code)]
#[path = "../../../../../src/hardware/broker/class.rs"]
mod class;
mod device;
#[path = "../../../../../src/hardware/broker/fdt_i2c/record.rs"]
mod fdt_i2c_record;
mod fdt_i2c_tests;

// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
//! driver.spi0: an Intel LPSS SPI host, its raw transfers served to one
//! client. Never the firmware's flash controller: the broker refuses that
//! claim, and this driver binds only a function whose LPSS capabilities
//! name SPI. Its endpoint is held to driver.spi_hid0 alone.

#![no_std]
#![no_main]

extern crate alloc;

mod core;
mod discover;
mod doorbell;
mod facts;
#[path = "../../capsule_driver_cdc_acm/src/frame.rs"]
mod frame;
/// driver.gpio0's EVENTS, asked for the holder's line under this host's key.
#[path = "../../capsule_driver_i2c_pci/src/gpio0/mod.rs"]
mod gpio0;
/// driver.i2c_pci0's table of LPSS I2C functions: never claimed here, so
/// this driver cannot take one from under the touchpad's driver.
#[path = "../../capsule_driver_i2c_pci/src/constants/device_info.rs"]
mod i2c_ids;
mod ops;
#[path = "../../capsule_driver_cdc_acm/src/acm/owner.rs"]
mod owner;
mod regs;
mod run;
mod setup;
mod transfer;
mod wire;

use nonos_libc::{heap_init, mk_exit, start_driver};

const DRIVER: &[u8] = b"driver.spi";

/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    match start_driver(DRIVER, discover::candidates(), setup::run) {
        Ok(host) => run::run(host),
        Err(code) => mk_exit(code),
    }
}

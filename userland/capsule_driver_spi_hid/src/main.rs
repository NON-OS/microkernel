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

//! driver.spi_hid0: HID over SPI, a touchpad or keyboard on driver.spi0's
//! bus as the firmware's serial-bus record names it. Its reports are
//! decoded and posted by the same code as HID over I2C (mounted): only how
//! a report is fetched differs. No hardware is held here; spi0 holds it.

#![no_std]
#![no_main]

extern crate alloc;

mod acpi;
mod bring_up;
mod feature;
// Shared with HID over I2C, whose frame dump this driver does not make.
#[allow(dead_code)]
#[path = "../../capsule_driver_i2c_hid/src/diag.rs"]
mod diag;
mod hid;
mod input;
mod link;
mod report;
mod run;
mod setup;
mod shape;
mod spi;
// Shared with HID over I2C: its bus address and register fields go unread.
#[allow(dead_code)]
#[path = "../../capsule_driver_i2c_hid/src/state/mod.rs"]
mod state;

use nonos_libc::{heap_init, mk_exit, say_absent, EXIT_ABSENT};

const DRIVER: &[u8] = b"driver.spi_hid";

/// # Safety
/// The kernel enters here once, on the capsule's own fresh stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    match setup::run() {
        Ok(link) => run::run(link, state::State::new(0, 0)),
        Err(why) => {
            diag::line(alloc::format!("{why}\n"));
            say_absent(DRIVER);
            mk_exit(EXIT_ABSENT)
        }
    }
}

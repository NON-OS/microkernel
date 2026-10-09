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
#![no_std]
#![no_main]

extern crate alloc;

use nonos_libc::{heap_init, mk_exit, say_absent, EXIT_ABSENT};

mod diag;
mod hid;
mod i2c_client;
mod input;
mod protocol;
mod server;
mod setup;
mod state;

const DRIVER: &[u8] = b"driver.i2c_hid";

/// The pad sits behind the I2C controller driver, so presence is that
/// driver's service. Setup fails only when it is not there within a short,
/// parked window (no controller on this machine, or one that never came up):
/// the driver says so and leaves with `EXIT_ABSENT`. A controller with no pad
/// on its bus is not a failure; the server keeps re-probing for one.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(1);
    }
    match setup::run() {
        Ok(state) => server::run(state),
        Err(_) => {
            say_absent(DRIVER);
            mk_exit(EXIT_ABSENT)
        }
    }
}

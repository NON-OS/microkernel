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

mod constants;
mod discover;
mod dispi;
mod display;
mod edid;
mod error;
mod gone;
mod handles;
mod regs;
mod serve;
mod setup;

use nonos_libc::{mk_exit, start_driver};

use crate::error::reason;

const DRIVER: &[u8] = b"driver.bga";

/// Without an adapter the driver says so and leaves (`EXIT_ABSENT`) before
/// claiming anything. An adapter that is there is set up on the shared
/// bounded schedule, and running out is `EXIT_GAVE_UP`. Once up, the
/// driver serves the display wire as `driver.bga0` until it is stopped.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let started =
        start_driver(DRIVER, discover::find_bga(), |dev| setup::run(*dev).map_err(reason));
    let mut driver = match started {
        Ok(driver) => driver,
        Err(code) => mk_exit(code),
    };
    let device_id = driver.handles.device_id();
    serve::run(&mut driver.server, device_id)
}

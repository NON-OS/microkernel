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
mod error;
mod handles;
mod regs;
mod setup;

use nonos_libc::{mk_exit, mk_idle_ms, start_driver};

use crate::error::reason;

const DRIVER: &[u8] = b"driver.bga";
/// The driver serves nothing once the mode is set; it lives only to hold the
/// claim and the framebuffer mapping. Each wakeup is wasted, so they are rare.
const HOLD_MS: u64 = 60_000;

/// Without an adapter the driver says so and leaves (`EXIT_ABSENT`) before
/// claiming anything. An adapter that is there is set up on the shared
/// bounded schedule and running out is `EXIT_GAVE_UP`. Once the mode is set
/// the driver sleeps: it used to call `mk_yield` in a loop, which returns at
/// once when nothing else is runnable and so held a core forever.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let started =
        start_driver(DRIVER, discover::find_bga(), |dev| setup::run(*dev).map_err(reason));
    let _driver = match started {
        Ok(driver) => driver,
        Err(code) => mk_exit(code),
    };
    loop {
        let _ = mk_idle_ms(HOLD_MS);
    }
}

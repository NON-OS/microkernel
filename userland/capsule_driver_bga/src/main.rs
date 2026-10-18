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
mod serve;
mod setup;

use nonos_libc::{mk_exit, start_driver};

use crate::error::reason;

/// The service was refused: the kernel would not register the name.
const EXIT_UNANNOUNCED: i32 = 1;

/// Without an adapter the driver says so and leaves (`EXIT_ABSENT`) before
/// claiming anything. An adapter that is there is set up on the shared
/// bounded schedule and running out is `EXIT_GAVE_UP`. Once it is up the
/// driver serves the compositor the display wire, waiting in its receive.
///
/// # Safety
///
/// The capsule's entry point: the kernel calls it once, on a fresh stack,
/// and nothing else may.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let started = start_driver(serve::SERVICE_NAME, discover::find_bga(), |dev| {
        setup::run(*dev).map_err(reason)
    });
    let driver = match started {
        Ok(driver) => driver,
        Err(code) => mk_exit(code),
    };
    if !serve::announce() {
        // The compositor finds the surface only through the service, so
        // nothing holds it: the adapter is given back showing the
        // firmware's mode before the capsule leaves, not left on grants
        // the exit frees.
        drop(driver);
        mk_exit(EXIT_UNANNOUNCED);
    }
    serve::run(driver)
}

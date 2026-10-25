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

mod announce;
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
mod turn;

use nonos_gfx_pool::Pool;
use nonos_libc::{heap_init, mk_exit, start_driver};
// `turn` reaches the DGON parser here, where its proofs mount the kernel's
// own definition under the same name.
use nonos_libc::dgon_parse;

use crate::error::reason;

/// The service was refused: the kernel would not register the name.
const EXIT_UNANNOUNCED: i32 = 1;
/// No heap: the display server allocates from the first request on.
const EXIT_HEAP_INIT: i32 = 1;

/// Without an adapter the driver says so and leaves (`EXIT_ABSENT`) before
/// claiming anything; one that is there is set up on the shared bounded
/// schedule (`EXIT_GAVE_UP` when it runs out), then `driver.bga0` is
/// announced and the display wire served until the driver is stopped.
///
/// # Safety
/// The kernel enters here once, on the capsule's own stack.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(EXIT_HEAP_INIT);
    }
    let started = start_driver(announce::SERVICE_NAME, discover::find_bga(), |dev| {
        setup::run(*dev, Pool::default()).map_err(reason)
    });
    let mut driver = match started {
        Ok(driver) => driver,
        Err(code) => mk_exit(code),
    };
    if !announce::announce() {
        // The compositor finds the display only through the service, so
        // nothing holds a primary: the adapter is given back in the firmware's
        // mode before the capsule leaves, not left on grants the exit frees.
        drop(driver);
        mk_exit(EXIT_UNANNOUNCED);
    }
    let device_id = driver.handles.device_id();
    serve::run(&mut driver.server, device_id)
}

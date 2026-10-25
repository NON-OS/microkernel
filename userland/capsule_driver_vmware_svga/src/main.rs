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

mod announce;
mod constants;
mod discover;
mod display;
mod fifo;
mod gone;
mod handles;
mod init;
mod regs;
mod say;
mod screen;
mod serve;
mod setup;
mod turn;

use nonos_gfx_pool::Pool;
use nonos_libc::{heap_init, mk_exit, start_driver};
// `turn` reaches the DGON parser here, where its proofs mount the kernel's
// own definition under the same name.
use nonos_libc::dgon_parse;

const DRIVER: &[u8] = b"driver.vmware_svga";
const EXIT_HEAP_INIT: i32 = 1;

/// Without an SVGA II adapter the driver says so and leaves before claiming
/// anything. With one, each attempt takes the grants, sets the mode and
/// starts the wire server, giving everything back and the firmware's mode
/// with it when a step fails; the attempts are bounded and slept between.
/// Once up, it announces `driver.vmware_svga0` and serves the display wire
/// until it is stopped.
///
/// # Safety
///
/// The capsule's entry point: the kernel calls it once, on a fresh stack,
/// and nothing else may.
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    if heap_init().is_err() {
        mk_exit(EXIT_HEAP_INIT);
    }
    let started =
        start_driver(DRIVER, discover::find_svga(), |dev| setup::run(*dev, Pool::default()));
    let driver = match started {
        Ok(d) => d,
        Err(code) => mk_exit(code),
    };
    say::up(&driver.server.display);
    let mut driver = announce::or_leave(driver);
    let device_id = driver.handles.device_id();
    serve::run(&mut driver.server, device_id)
}
